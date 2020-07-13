#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    for i in 0..n {
        if s[i..] == t[..n - i] {
            println!("{}", i + n);
            return;
        }
    }
    println!("{}", n * 2);
}
