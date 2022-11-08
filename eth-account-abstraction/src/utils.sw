library utils;

use std::{
    b512::B512,
    ecr::{ec_recover, EcRecoverError},
    hash::{
        keccak256,
        sha256,
    },
    identity::Identity,
    vm::evm::evm_address::EvmAddress,
};


//Applies the prefix used by Geth to a message hash.
//Returns the prefixed hash.
pub fn eth_prefix(msg_hash: b256) -> b256 {

    let prefix = "\x19Ethereum Signed Message:\n32";

    sha256((prefix, msg_hash))
}


//Creates an EIP-191 compliant transaction hash, of the version:
//0x45, personal sign.
//It takes a data_to_sign to represent the <data to sign> in the following EIP-191 format:
//0x19 <1 byte version> <version specific data> <data to sign>
pub fn eip_191_format(data_to_sign: b256) -> b256 {

    let initial_byte= 0x19u8;
    let version_byte= 0x45u8;

    let packed_bytes = compose((initial_byte, version_byte, 0 ,0));

    let encoded_data = encode_data(packed_bytes, data_to_sign);
    let encoded_data = (
        encoded_data.get(0).unwrap(),
        encoded_data.get(1).unwrap(),
        encoded_data.get(2).unwrap(),
        encoded_data.get(3).unwrap(),
        encoded_data.get(4).unwrap(),
    );

    //Keccak256 hash the first 34 bytes of encoded_data
    let mut result_buffer: b256 = b256::min();

    asm(hash: result_buffer, ptr: encoded_data, bytes: 34) {
        k256 hash ptr bytes; // Hash the next "bytes" number of bytes starting from "ptr", into "hash"
        hash: b256 // Return hash as b256
    }
    
}


// Build a single b256 value from a tuple of 4 u64 values.
pub fn compose(words: (u64, u64, u64, u64)) -> b256 {

    asm(r1: __addr_of(words)) { r1: b256 }

}


// Get a tuple of 4 u64 values from a single b256 value.
pub fn decompose(val: b256) -> (u64, u64, u64, u64) {

    asm(r1: __addr_of(val)) { r1: (u64, u64, u64, u64) }

}


// Encode the packed_bytes and message_hash into a Vec<u64> of length 40 bytes,
// where the first 34 bytes are the desired data
fn encode_data(packed_bytes: b256, message_hash: b256) -> Vec<u64> {

    let mut data = Vec::with_capacity(5);
    
    let (bytes_1, bytes_2, _bytes_3, _bytes_4) = decompose(packed_bytes);
    let (message_1, message_2, message_3, message_4) = decompose(message_hash);

    data.push(
        (bytes_1 << 56) + 
        (bytes_2 << 48) +
        (message_1 >> 16)
        );

    data.push(
        (message_1 << 48) +
        (message_2 >> 16)
    );
    data.push(
        (message_2 << 48) +
        (message_3 >> 16)
    );
    data.push(
        (message_3 << 48) +
        (message_4 >> 16)
    );
    data.push(
        message_4 << 48
    );

    data
}
