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

fn main() {
    skip_line();
    let points = get_ns::<i64>();

    let mut offset = 0;
    for mut p in points {
        p -= offset;
        offset = -p;
    }

    println!("{offset}");
}

// 11m
