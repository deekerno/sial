#![allow(dead_code)]

use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::error::Error;
use std::default::Default;

mod tokenizer;      // Tokenizes files into words, sentences, and paragraphs.

#[derive(Default, Copy, Clone)]
struct Metadata {
    wordlen: f32,	// The length of each of these characteristics will be
    sentlen: f32,	// averaged and compared with other source.
    paralen: i32,
    comma: i32,
    semicolon: i32,
    quote: i32,
    bangs: i32,
    dashes: i32,
    ands: i32,
    buts: i32,
    however: i32,
    condition: i32,		// Prevent confusion with if.
    thats: i32,
    more: i32,
    musts: i32,
    mights: i32,
    thises: i32,
    very: i32,
}

fn main() {
    
    let mut off = Metadata {..Default::default()};
    let mut pseu = Metadata {..Default::default()};

    let args: Vec<String> = env::args().collect();
    
    let mut file1 = match File::open(&args[1]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[1], Error::description(&why)),
        Ok(file1) => file1,
    };
    let mut file2 = match File::open(&args[2]) {
        Err(why) => panic!("Couldn't open {}: {}", &args[2], Error::description(&why)),
        Ok(file2) => file2,
    };

    let (mut s1, mut s2) = (String::new(), String::new());

    match file1.read_to_string(&mut s1) {
        Err(why) => panic!("Couldn't read file1: {}", Error::description(&why)),
        Ok(_) => println!("File 1 read."),
    };
    
    match file2.read_to_string(&mut s2) {
        Err(why) => panic!("Couldn't read file2: {}", Error::description(&why)),
        Ok(_) => println!("File 2 read."),
    };

}