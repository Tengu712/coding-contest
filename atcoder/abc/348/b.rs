#![allow(dead_code)]

fn main() {
    let n = get_n::<u64>();

    let mut ps = Vec::new();
    for _ in 0..n {
        let xy = get_ns::<i64>();
        ps.push((xy[0], xy[1]));
    }

    for i in 0..ps.len() {
        let mut max = 0;
        let mut max_idx = 0;
        for j in 0..ps.len() {
            let p1 = ps[i];
            let p2 = ps[j];
            let dis = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2);
            if max < dis {
                max = dis;
                max_idx = j;
            }
        }
        println!("{}", max_idx + 1);
    }
}

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
