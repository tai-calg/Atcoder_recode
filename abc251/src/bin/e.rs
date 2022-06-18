use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp1 = vec![vec![0; 2]; n];
    dp1[0][0] = 0;
    dp1[0][1] = std::usize::MAX;
    for i in 1..n {
        dp1[i][0] = dp1[i-1][1];
        dp1[i][1] = min(dp1[i-1][0], dp1[i-1][1]) + a[i];
    }
    let sum1 = dp1[n-1][1];//最後に行動しなければならない場合

    let mut dp2 = vec![vec![0; 2]; n];
    dp2[0][0] = std::usize::MAX;
    dp2[0][1] = a[0];
    for i in 1..n {
        dp2[i][0] = dp2[i-1][1];
        dp2[i][1] = min(dp2[i-1][0], dp2[i-1][1]) + a[i];
    }
    let sum2 = min(dp2[n-1][0], dp2[n-1][1]);//0でこうどうしてるのでしてもしなくてもよい場合

    let ans = min(sum1, sum2);
    println!("{}", ans);
}