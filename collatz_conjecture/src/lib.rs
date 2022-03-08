// Copyright 2021 Andrew Merenbach
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::iter;

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
    iter::successors(Some(seed), |&n| match n {
        n if n == m_0 => None,
        n if n % d == 1 => Some(m_1 * n - r_1),
        _ => Some((m_0 * n - r_0) / d), // TODO: is this correct placement for m_0 and r_0?
    })
    .collect()
}

pub fn collatz(seed: i64) -> Vec<i64> {
    return collatz_generalized(seed, 2, 1, 3, 0, -1);
}
