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
    let source = AutoSource::from("30 5
325 234 123
rspsspspsrpspsppprpsprpssprpsr
");
    input!{
        // from source,
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        t: Chars
    };

    let _point = vec![p, s, r];

    let mut v = vec![vec![]; k];

    for j in 0..n {
        v[j % k].push(t[j])
        // v[j % k].push(match t[j]{
        //     'r' => 0usize,
        //     's' => 1,
        //     'p' => 2,
        //     _ => unreachable!()
        // });
    }

    // println!("{:?}", &v);

    let mut ans = 0;

    for u in v{
        let mut dp = vec![vec![0; 3];u.len()+1];

        for i in 0..u.len(){
            dp[i+1][0] = max(dp[i][1] ,dp[i][2]) + if u[i] == 's' {r} else {0};
            dp[i+1][1] = max(dp[i][2] ,dp[i][0]) + if u[i] == 'p' {s} else {0};
            dp[i+1][2] = max(dp[i][0] ,dp[i][1]) + if u[i] == 'r' {p} else {0};
        }

        // println!("{:?}", dp);

        ans += max(dp[dp.len() - 1][0], max(dp[dp.len() - 1][1], dp[dp.len() - 1][2]));
    }

    println!("{}", ans);

}
