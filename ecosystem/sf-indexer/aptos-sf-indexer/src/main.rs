// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! Indexer is used to index blockchain data into Postgres
//!
//! TODO: Examples
//!
#![forbid(unsafe_code)]

use aptos_logger::{error, info};
use aptos_sf_indexer::indexer::substream_processor::{
    get_start_block, run_migrations, SubstreamProcessor,
};
use aptos_sf_indexer::proto;

use anyhow::{format_err, Context, Error};
use aptos_sf_indexer::database::new_db_pool;
use aptos_sf_indexer::{
    substream_processors::block_output_processor::BlockOutputSubstreamProcessor,
    substreams::SubstreamsEndpoint,
    substreams_stream::{BlockResponse, SubstreamsStream},
};
use clap::Parser;
use futures::StreamExt;
use prost::Message;
use std::{env, sync::Arc};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct IndexerArgs {
    // URL of the firehose gRPC endpoint
    #[clap(long)]
    endpoint_url: String,

    // Relative location of the substream wasm file (.spkg)
    #[clap(long)]
    package_file: String,

    // Substream module name
    #[clap(long)]
    module_name: String,

    /// If set, don't run any migrations
    #[clap(long)]
    skip_migrations: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    aptos_logger::Logger::new().init();
    let args: IndexerArgs = IndexerArgs::parse();
    info!("Starting indexer...");

    let endpoint_url = &args.endpoint_url;
    let package_file = &args.package_file;
    let substream_module_name = &args.module_name;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn_pool = new_db_pool(&database_url).unwrap();
    info!("Created the connection pool... ");

    if !args.skip_migrations {
        run_migrations(&conn_pool);
    }

    let token_env = env::var("SUBSTREAMS_API_TOKEN").unwrap_or_else(|_| "".to_string());
    let mut token: Option<String> = None;
    if !token_env.is_empty() {
        token = Some(token_env);
    }
    let package = read_package(package_file)?;
    let endpoint = Arc::new(SubstreamsEndpoint::new(&endpoint_url, token).await?);

    info!("Created substream endpoint");
    let start_block = get_start_block(&conn_pool, substream_module_name).unwrap_or_else(|| {
        info!("Could not fetch max block so starting from block 0");
        0
    });
    info!("Starting stream from block {}", start_block);

    let mut stream = SubstreamsStream::new(
        endpoint.clone(),
        None, // We're using block instead of cursor currently
        package.modules.clone(),
        substream_module_name.to_string(),
        start_block,
        start_block + 500,
    );

    let mut block_height = start_block as u64;
    loop {
        match stream.next().await {
            None => {
                info!("Stream consumed for module {}", substream_module_name);
                break;
            }
            Some(event) => {
                if let Ok(BlockResponse::New(data)) = event {
                    info!(
                        "Consuming module output (module {}, block {}, cursor {})",
                        substream_module_name, block_height, data.cursor
                    );

                    if substream_module_name == "block_to_block_output" {
                        let mut processor = BlockOutputSubstreamProcessor::new(conn_pool.clone());
                        match processor
                            .process_substream_with_status(
                                substream_module_name.clone(),
                                data,
                                block_height,
                            )
                            .await
                        {
                            Ok(_) => {
                                info!("Finished processing block {}", block_height);
                                block_height += 1
                            }
                            Err(error) => {
                                error!(
                                    "Error processing block {}, error: {:?}",
                                    block_height, &error
                                );
                                panic!();
                            }
                        };
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_package(file: &str) -> Result<proto::Package, anyhow::Error> {
    let content = std::fs::read(file).context(format_err!("read package {}", file))?;
    proto::Package::decode(content.as_ref()).context("decode command")
}