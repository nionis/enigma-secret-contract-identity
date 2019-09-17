#![no_std]
#![allow(unused_attributes)]

extern crate eng_wasm;
extern crate eng_wasm_derive;
extern crate enigma_crypto;
extern crate serde;
extern crate serde_derive;
extern crate rustc_hex;

use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use enigma_crypto::asymmetric::KeyPair;
use serde::{Deserialize, Serialize};
use rustc_hex::ToHex;

static STATE: &str = "STATE";

#[derive(Serialize, Deserialize)]
struct State {
  private_key: [u8; 32],
  nonce: U256
}

#[pub_interface]
pub trait ContractInterface {
  fn construct() -> ();
  fn get_public_key() -> String;
  fn get_nonce() -> U256;
}

impl Contract {
  fn get_state() -> State {
    match read_state!(STATE).unwrap() {
      Some(state) => state,
      None => panic!("state should already exist"),
    }
  }
}

pub struct Contract;
impl ContractInterface for Contract {
  fn construct() -> () {
    write_state!(STATE => State {
      private_key: generate_key(),
      nonce: U256::from(0)
    });
  }

  fn get_public_key() -> String {
    let private_key = Self::get_state().private_key;
    let key_pair = KeyPair::from_slice(&private_key).unwrap();
    let public_key = key_pair.get_pubkey();

    return public_key.to_hex::<String>();
  }

  fn get_nonce() -> U256 {
    return Self::get_state().nonce;
  }
}
