#![allow(dead_code, unused_imports)]

use std::collections::*;

/// A funtion to skip one line.
fn skip_line() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

/// A function that reads a single line of standard input and returns it.
fn get_s() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

/// A function that reads a single line of standard input containing one value,
/// parses it, and returns the parsed value.
fn get_n<T>() -> T
where
    T: std::str::FromStr,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse::<T>().ok().unwrap()
}

/// A function that reads a single line of standard input containing multiple
/// values separated by whitespace, parses them, and returns the parsed values.
fn get_ns<T>() -> Vec<T>
where
    T: std::str::FromStr,
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim()
        .split(' ')
        .map(|n| n.parse::<T>().ok().unwrap())
        .collect::<Vec<T>>()
}

fn get_min(b: u64) -> u64 {
    if b < 2 {
        return 0;
    }
    let mut res = 0;
    let mut a = 4;
    let mut mode = true;
    for i in 2..(b + 1) {
        res += 1;
        if i >= a {
            res += 1;
            a += if mode { 2 } else { 3 };
            mode = !mode;
        }
    }
    res
}

fn get_max(b: u64) -> u64 {
    let mut res = 2;
    let mut a = 2;
    let mut mode = true;
    for i in 1..(b + 1) {
        res += 1;
        if i >= a {
            res += 1;
            a += if mode { 3 } else { 2 };
            mode = !mode;
        }
    }
    res
}

fn main() {
    let wb = get_ns::<u64>();
    let w = wb[0];
    let b = wb[1];

    if w < b {
        if b - w < 2 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if get_min(b) <= w && w <= get_max(b) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
