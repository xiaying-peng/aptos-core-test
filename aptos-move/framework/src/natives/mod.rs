// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

pub mod account;
pub mod bit_vector;
pub mod code;
pub mod cryptography;
pub mod event;
pub mod hash;
mod helpers;
pub mod ristretto255;
pub mod ristretto255_point;
pub mod ristretto255_scalar;
pub mod signature;
pub mod transaction_context;
pub mod type_info;
pub mod util;

use move_deps::{
    move_core_types::{account_address::AccountAddress, identifier::Identifier},
    move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable},
};

pub mod status {
    // Failure in parsing a struct type tag
    pub const NFE_EXPECTED_STRUCT_TYPE_TAG: u64 = 0x1;
    // Failure in address parsing (likely no correct length)
    pub const NFE_UNABLE_TO_PARSE_ADDRESS: u64 = 0x2;
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub account: account::GasParameters,
    pub bit_vector: bit_vector::GasParameters,
    pub signature: signature::GasParameters,
    pub bls12381: cryptography::bls12381::GasParameters,
    pub ristretto255: ristretto255::GasParameters,
    pub hash: hash::GasParameters,
    pub type_info: type_info::GasParameters,
    pub util: util::GasParameters,
    pub transaction_context: transaction_context::GasParameters,
    pub code: code::GasParameters,
    pub event: event::GasParameters,
}

impl GasParameters {
    pub fn zeros() -> Self {
        Self {
            account: account::GasParameters {
                create_address: account::CreateAddressGasParameters { base_cost: 0 },
                create_signer: account::CreateSignerGasParameters { base_cost: 0 },
            },
            bls12381: cryptography::bls12381::GasParameters {
                base_cost: 0,
                per_pubkey_deserialize_cost: 0,
                per_pubkey_aggregate_cost: 0,
                per_pubkey_subgroup_check_cost: 0,
                per_sig_deserialize_cost: 0,
                per_sig_aggregate_cost: 0,
                per_sig_subgroup_check_cost: 0,
                per_sig_verify_cost: 0,
                per_pop_verify_cost: 0,
                per_pairing_cost: 0,
                per_msg_hashing_cost: 0,
                per_byte_hashing_cost: 0,
            },
            bit_vector: bit_vector::GasParameters {
                little_endian_bitvector_from_byte_vector:
                    bit_vector::LittleEndianBitVectorFromByteVectorGasParams {
                        base_cost: 0,
                        per_byte_cost: 0,
                    },
                big_endian_bitvector_from_byte_vector:
                    bit_vector::BigEndianBitVectorFromByteVectorGasParams {
                        base_cost: 0,
                        per_byte_cost: 0,
                    },
            },
            ristretto255: ristretto255::GasParameters {
                point_is_canonical: ristretto255_point::PointIsCanonicalGasParameters {
                    base_cost: 0,
                    is_canonical_cost: 0,
                },
                point_identity: ristretto255_point::PointIdentityGasParameters { base_cost: 0 },
                point_decompress: ristretto255_point::PointDecompressGasParameters {
                    base_cost: 0,
                    decompress_cost: 0,
                },
                point_compress: ristretto255_point::PointCompressGasParameters { base_cost: 0 },
                point_mul: ristretto255_point::PointMulGasParameters { base_cost: 0 },
                point_equals: ristretto255_point::PointEqualsGasParameters { base_cost: 0 },
                point_neg: ristretto255_point::PointNegGasParameters { base_cost: 0 },
                point_add: ristretto255_point::PointAddGasParameters { base_cost: 0 },
                point_sub: ristretto255_point::PointSubGasParameters { base_cost: 0 },
                scalar_is_canonical: ristretto255_scalar::ScalarIsCanonicalGasParameters {
                    base_cost: 0,
                    per_scalar_deserialize_cost: 0,
                },
                scalar_invert: ristretto255_scalar::ScalarInvertGasParameters {
                    base_cost: 0,
                    per_scalar_invert_cost: 0,
                },
                scalar_from_sha512: ristretto255_scalar::ScalarFromSha512GasParameters {
                    base_cost: 0,
                    per_hash_sha512_cost: 0,
                    per_byte_sha512_cost: 0,
                },
                scalar_mul: ristretto255_scalar::ScalarMulGasParameters {
                    base_cost: 0,
                    mul_cost: 0,
                },
                scalar_add: ristretto255_scalar::ScalarAddGasParameters {
                    base_cost: 0,
                    add_cost: 0,
                },
                scalar_sub: ristretto255_scalar::ScalarSubGasParameters {
                    base_cost: 0,
                    sub_cost: 0,
                },
                scalar_neg: ristretto255_scalar::ScalarNegGasParameters {
                    base_cost: 0,
                    neg_cost: 0,
                },
                scalar_from_u64: ristretto255_scalar::ScalarFromU64GasParameters {
                    base_cost: 0,
                    from_u64_cost: 0,
                },
                scalar_from_u128: ristretto255_scalar::ScalarFromU128GasParameters {
                    base_cost: 0,
                    from_u128_cost: 0,
                },
                scalar_from_256_bits: ristretto255_scalar::ScalarFrom256BitsGasParameters {
                    base_cost: 0,
                    from_256_bits_cost: 0,
                },
                scalar_from_512_bits: ristretto255_scalar::ScalarFrom512BitsGasParameters {
                    base_cost: 0,
                    from_512_bits_cost: 0,
                },
            },
            signature: signature::GasParameters {
                // Ed25519
                ed25519_validate_pubkey: signature::Ed25519ValidatePubkeyGasParameters {
                    base_cost: 0,
                    per_pubkey_deserialize_cost: 0,
                    per_pubkey_small_order_check_cost: 0,
                },
                ed25519_verify: signature::Ed25519VerifyGasParameters {
                    base_cost: 0,
                    per_pubkey_deserialize_cost: 0,
                    per_sig_deserialize_cost: 0,
                    per_sig_strict_verify_cost: 0,
                    per_msg_hashing_base_cost: 0,
                    per_msg_byte_hashing_cost: 0,
                },

                // secp256k1
                secp256k1_ecdsa_recover: signature::Secp256k1ECDSARecoverGasParameters {
                    base_cost: 0,
                },
            },
            hash: hash::GasParameters {
                sip_hash: hash::SipHashGasParameters {
                    base_cost: 0,
                    unit_cost: 0,
                },
            },
            type_info: type_info::GasParameters {
                type_of: type_info::TypeOfGasParameters {
                    base_cost: 0,
                    unit_cost: 0,
                },
                type_name: type_info::TypeNameGasParameters {
                    base_cost: 0,
                    unit_cost: 0,
                },
            },
            util: util::GasParameters {
                from_bytes: util::FromBytesGasParameters {
                    base_cost: 0,
                    unit_cost: 0,
                },
            },
            transaction_context: transaction_context::GasParameters {
                get_script_hash: transaction_context::GetScriptHashGasParameters { base_cost: 0 },
            },
            code: code::GasParameters {
                request_publish: code::RequestPublishGasParameters {
                    base_cost: 0,
                    unit_cost: 0,
                },
            },
            event: event::GasParameters {
                write_to_event_store: event::WriteToEventStoreGasParameters { unit_cost: 0 },
            },
        }
    }
}

pub fn all_natives(
    framework_addr: AccountAddress,
    gas_params: GasParameters,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives_from_module {
        ($module_name: expr, $natives: expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives_from_module!("account", account::make_all(gas_params.account));
    add_natives_from_module!("bit_vector", bit_vector::make_all(gas_params.bit_vector));
    add_natives_from_module!("signature", signature::make_all(gas_params.signature));
    add_natives_from_module!(
        "bls12381",
        cryptography::bls12381::make_all(gas_params.bls12381)
    );
    add_natives_from_module!("aptos_hash", hash::make_all(gas_params.hash));
    add_natives_from_module!(
        "ristretto255",
        ristretto255::make_all(gas_params.ristretto255)
    );
    add_natives_from_module!("type_info", type_info::make_all(gas_params.type_info));
    add_natives_from_module!("util", util::make_all(gas_params.util));
    add_natives_from_module!(
        "transaction_context",
        transaction_context::make_all(gas_params.transaction_context)
    );
    add_natives_from_module!("code", code::make_all(gas_params.code));
    add_natives_from_module!("event", event::make_all(gas_params.event));

    make_table_from_iter(framework_addr, natives)
}

/// A temporary hack to patch Table -> table module name as long as it is not upgraded
/// in the Move repo.
pub fn patch_table_module(table: NativeFunctionTable) -> NativeFunctionTable {
    table
        .into_iter()
        .map(|(m, _, f, i)| (m, Identifier::new("table").unwrap(), f, i))
        .collect()
}
