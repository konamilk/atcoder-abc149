use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        x: usize
    };

    let primes =eratosthenes(1_000_000);

    let idx = primes.lower_bound(|&k| k >= x);

    println!("{}", primes[idx])

}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut table = vec!(true; n+1);
    let mut primes = vec![];
    table[0] = false;
    table[1] = false;
    for i in 2..=sqrt(n) {
        if table[i] == false {
            continue;
        }
        let mut j = 2*i;
        while j <= n {
            table[j] = false;
            j = j + i;
        }
    }

    for (i, is_prime) in table.into_iter().enumerate(){
        if is_prime { primes.push(i)}
    }

    primes
}

#[test]
fn eratosthenes_test() {
    assert_eq!(eratosthenes(11), vec![2,3,5,7,11]);
    assert_eq!(eratosthenes(12), vec![2,3,5,7,11]);
    assert_eq!(eratosthenes(16), vec![2,3,5,7,11,13]);
}


trait BinarySearch<T> {
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize;
}

impl<T> BinarySearch<T> for [T] {
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize {
        let mut left: isize = -1;
        let mut right = self.len() as isize;

        while right - left > 1 {
            let mid = (left + right) / 2;
            if f(&self[mid as usize]) {
                right = mid;
            } else {
                left = mid;
            }
        }

        right as usize
    }
}


#[test]
fn binary_search_test() {
    assert_eq!(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0].lower_bound(|&k| k > 5),
        5
    );
    assert_eq!(
        vec![2, 6, 4, 12, 8, 24].lower_bound(|&k| k % (2 * 2 * 2) == 0),
        4
    );
    assert_eq!(vec![1, 2, 3, 4, 5].lower_bound(|&k| k > 0), 0);
    assert_eq!(vec![1, 2, 3, 4, 5].lower_bound(|&k| k > 5), 5);
}

