#![allow(non_snake_case, dead_code)]

extern crate bn;
extern crate rand;
extern crate snark;
extern crate crossbeam;
extern crate rustc_serialize;
extern crate blake2_rfc;
extern crate bincode;
extern crate byteorder;

#[macro_use]
mod protocol;

mod consts;
use self::consts::*;

use std::fs::File;
use protocol::*;
use snark::*;

use bincode::SizeLimit::Infinite;
use bincode::rustc_serialize::{encode_into, decode_from};

pub const THREADS: usize = 128;

fn main() {
    let mut f = File::open("transcript").unwrap();
    let cs = {
        if USE_DUMMY_CS {
            CS::dummy()
        } else {
            CS::from_file()
        }
    };
    let num_players: usize = decode_from(&mut f, Infinite).unwrap();
    println!("Number of players: {}", num_players);
    
    let mut commitments = vec![];
    let mut pubkeys = vec![];
    for i in 0..num_players {
        let comm: Digest256 = decode_from(&mut f, Infinite).unwrap();
        commitments.push(comm);
        println!("Player {} commitment: {}", i+1, comm.to_string());
    }

    let mut stage1 = Stage1Contents::new(&cs);

    for i in 0..num_players {
        println!("Player {} in stage 1", i+1);
        let pubkey: PublicKey = decode_from(&mut f, Infinite).unwrap();

        let nizks: PublicKeyNizks = decode_from(&mut f, Infinite).unwrap();

        let new_stage: Stage1Contents = decode_from(&mut f, Infinite).unwrap();

        let ihash: Digest256 = decode_from(&mut f, Infinite).unwrap();

        stage1 = new_stage;
        pubkeys.push(pubkey);
        println!("Player {} done with stage 1", i+1);
    }
    println!("getting stage 2");    

    for i in 0..num_players-1 {
        println!("Player {} in stage 2", i+1);

        let new_stage: Stage2Contents = decode_from(&mut f, Infinite).unwrap();

        let ihash: Digest256 = decode_from(&mut f, Infinite).unwrap();
    }

    println!("Player 6 stage 2");
    let mut stage2: Stage2Contents = decode_from(&mut f, Infinite).unwrap();
    let ihash: Digest256 = decode_from(&mut f, Infinite).unwrap();

    for i in 0..num_players-1 {
        println!("Player {} in stage 3", i+1);
    }
    println!("player 6 stage 3!");
    let mut stage3: Stage3Contents = decode_from(&mut f, Infinite).unwrap();
    let ihash: Digest256 = decode_from(&mut f, Infinite).unwrap();
    
    let kp = keypair(&cs, &stage1, &stage2, &stage3);
    kp.write_to_disk();

}