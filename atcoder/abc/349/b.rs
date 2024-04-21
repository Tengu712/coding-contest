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
    let s = get_s();

    let mut appear_counts = HashMap::new();
    for c in s.chars() {
        if let Some(n) = appear_counts.get(&c) {
            appear_counts.insert(c, n + 1);
        } else {
            appear_counts.insert(c, 1);
        }
    }

    let mut kind_counts = HashMap::new();
    for (_, count) in appear_counts {
        if let Some(n) = kind_counts.get(&count) {
            kind_counts.insert(count, n + 1);
        } else {
            kind_counts.insert(count, 1);
        }
    }

    for (_, count) in kind_counts {
        if count != 0 && count != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

// 10m
