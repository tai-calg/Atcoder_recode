use proconio::input;
//https://atcoder.jp/contests/abc245/tasks/abc245_c
fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    let mut conds = vec![vec![true; 2]; n];
    for i in 0..n - 1 {
        conds[i + 1][0] = 
            conds[i][0] && (a[i] - a[i + 1]).abs() <= k || conds[i][1] && (b[i] - a[i + 1]).abs() <= k;

        conds[i + 1][1] = 
            conds[i][0] && (a[i] - b[i + 1]).abs() <= k || conds[i][1] && (b[i] - b[i + 1]).abs() <= k;
    }


    if conds[n - 1][0] || conds[n - 1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}

/* 
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:isize, k:isize, A:[isize;n], B:[isize;n]}


    if dfs(A[0],&A, &B, 0, n, k) || dfs(B[0],&B, &A, 0, n, k) {
        println!("Yes" );
    }else {
        println!("No" );
    }

}

fn dfs(x:isize ,A: &Vec<isize>, B: &Vec<isize>, i: isize,n:isize, k:isize)->bool {

    if i == n-1 {
        return true;
    }
    if (x - A[(i+1) as usize]).abs()  <= k {
        return dfs(A[(i+1) as usize ], A, B, i+1,n,k);
    }else if  (x - B[(i+1) as usize]).abs() <= k {
        return dfs(B[(i+1) as usize ], A, B, i+1,n,k);
    }else{
        return false;
    }



}
*/