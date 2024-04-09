#![allow(dead_code)]

fn main() {}

/// A funtion to skip one line.
fn skip_line() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
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
