#![allow(dead_code, unused_imports)]

use std::collections::*;

/*****************************************************************************/
/*   solve                                                                   */
/*****************************************************************************/

fn main() {
    let s = get_s();

    let mut set = HashSet::new();
    for i in 0..s.len() {
        for j in (i + 1)..(s.len() + 1) {
            set.insert(&s[i..j]);
        }
    }
    println!("{}", set.len());
}

/*****************************************************************************/
/*   libray                                                                  */
/*****************************************************************************/

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
