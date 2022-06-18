use itertools::Itertools;
use proconio::input;

fn main() {
    // n進法
    input! { w: usize }

    let mut ans = vec![];
    for i in (1..100).step_by(1) { ans.push(i); }
    for i in (100..10000).step_by(100) { ans.push(i); }
    for i in (10000..1000000).step_by(10000) { ans.push(i); }

    println!("{}\n{}", ans.len(), ans.iter().join(" "));
}