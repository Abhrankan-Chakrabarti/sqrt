use anyhow::anyhow;
use malachite::{
    Integer,
    base::num::arithmetic::traits::{Square, FloorSqrt, Pow},
};
use std::{io, io::Write, env};

macro_rules! input {
    ($prompt: literal) => {
        {
            print!($prompt);
            flush();
            let mut n = String::new();
            io::stdin().read_line(&mut n)?;
            let ret: anyhow::Result<String> = Ok(n.trim().to_string());
            ret
        }
    };
}

fn int(n: String) -> anyhow::Result<u64> {
    n.parse().map_err(|_| anyhow!("Unable to parse integer"))
}

fn print_usage(bin_path: String) {
    println!("Usage: {bin_path} [x] [digits]");
}

fn flush() {
    io::stdout().flush().unwrap();
}

fn printchar(c: &str, n: u64) {
    for _ in 0..n {
        print!("{c}");
    }
}

fn sqrt(x: u64, one: &Integer) -> Integer {
    (Integer::const_from_unsigned(x) * one.square()).floor_sqrt()
}

fn print_bigint(n: &Integer, digits: u64, one: &Integer) {
    print!("{}", n / one);
    if digits > 0 {
        print!(".");
        flush();
        print_strzfill(n % one, digits, one);
    }
}

fn print_strzfill(n: Integer, width: u64, one: &Integer) {
    let n_str = n.to_string();
    if &n * Integer::const_from_unsigned(10) < *one {
        printchar("0", width - n_str.len() as u64);
    }
    print!("{n_str}");
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let x: u64;
    let digits: u64;
    match args.len() {
        1 => {
            x = int(input!("Enter the number :\t")?)?;
            digits = int(input!("How many digits of √{x}? :\t")?)?;
        },
        3 => {
            x = int(args[1].clone())?;
            digits = int(args[2].clone())?;
        },
        _ => {
            print_usage(args[0].clone());
            return Err(anyhow!("Invalid number of arguments"));
        }
    }
    let one: Integer = Integer::const_from_unsigned(10).pow(digits);
    let sqrtx = sqrt(x, &one);
    print!("√{x} = ");
    print_bigint(&sqrtx, digits, &one);
    println!("...");
    Ok(())
}
