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

fn collatz_generalized(seed: i64, d: i64, m_0: i64, m_1: i64, r_0: i64, r_1: i64) -> Vec<i64> {
    successors(Some(seed), |&n| match n {
        n if n == m_0 => None,
        n if n % d == 1 => Some(m_1 * n - r_1),
        _ => Some((m_0 * n - r_0) / d), // TODO: is this correct placement for m_0 and r_0?
    })
    .collect()
}

fn collatz(seed: i64) -> Vec<i64> {
    return collatz_generalized(seed, 2, 1, 3, 0, -1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = args[1].parse::<i64>().unwrap();

    for out in collatz(seed) {
        println!("{}", out);
    }
}
