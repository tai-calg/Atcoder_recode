
use proconio::{input, fastout};

#[fastout]
fn main() {
    //入力回数が入力に依存
    input! {n :usize};
    let mut dvec:Vec<u16> = Vec::new();
    for _ in 0..n {
        input! {di: u16};
        dvec.push(di);
    }

    dvec.sort();
    dvec.dedup();

    println!("{}",dvec.len());
}
