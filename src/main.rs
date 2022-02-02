use std::env;
use std::iter::successors;

fn collatz(i: u64) -> Option<u64> {
    match i {
        i if i == 1 => None,
        i if i % 2 == 1 => Some(3 * i + 1),
        _ => Some(i / 2),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = args[1].parse::<u64>().unwrap();

    for out in successors(Some(seed), |&n| collatz(n)) {
        println!("{}", out);
    }
}
