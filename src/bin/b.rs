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
        a: i64,
        b: i64,
        k: i64
    };

    if k >= a+b{
        println!("0 0");
    }
    else if k <= a{
        println!("{} {}", a-k, b);
    }
    else {
        println!("{} {}", "0", a+b-k);
    }
}
