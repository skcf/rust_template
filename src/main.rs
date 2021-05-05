#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_macros)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use std::process::exit;

macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

macro_rules! p {
    ($x:expr) => {
        println!("{}", $x);
    };
}

macro_rules! pp {
    ($x:expr) => {
        println!("{:?}", $x);
    };
}

fn main() {
    input! {
        n: usize,
    };

    let mut ans: usize = 0;
    p!(ans);
}
