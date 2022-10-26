predicate;

dep utils;

use std::{
    b512::B512, 
    constants::ZERO_B256, 
    ecr::ec_recover_address, 
    inputs::input_predicate_data,
    vm::evm::{
        evm_address::EvmAddress,
        ecr::ec_recover_evm_address,
    },
};

fn recover_and_match(signature: B512, expected_address: b256) -> u64 {
    // if let Result::Ok(address) = ec_recover_address(signature, ZERO_B256)
    if let Result::Ok(address) = ec_recover_evm_address(signature, ZERO_B256)
    {
        if address.value == expected_address {
            return 1;
        }
    }
    0
}

fn main() -> bool {
    let signature: [B512; 1] = input_predicate_data(0);

    let spender_address = [
        // 0xd58573593432a30a800f97ad32f877425c223a9e427ab557aab5d5bb89156db0,//fuel address
        0x000000000000000000000000f2ec609af07cc6eec88e29a170299dd5de7e51e6//evm address
    ];

    let mut matched_addresses = 0;

    matched_addresses = recover_and_match(signature[0], spender_address[0]);

    matched_addresses > 0
}