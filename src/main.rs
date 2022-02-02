use std::env;
use std::iter::successors;

#[cfg(test)]
mod tests {
    use super::collatz;
    #[test]
    fn test_collatz() {
        assert_eq!(collatz(1), [1]);
        assert_eq!(collatz(2), [2, 1]);
        assert_eq!(collatz(3), [3, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(collatz(4), [4, 2, 1]);
        assert_eq!(
            collatz(227),
            [227, 682, 341, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]
        );
    }
}

fn collatz(i: u64) -> Vec<u64> {
    successors(Some(i), |&n| match n {
        n if n == 1 => None,
        n if n % 2 == 1 => Some(3 * n + 1),
        _ => Some(n / 2),
    })
    .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = args[1].parse::<u64>().unwrap();

    for out in collatz(seed) {
        println!("{}", out);
    }
}
