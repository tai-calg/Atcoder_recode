#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::HashMap;

use itertools::Itertools;
use proconio::*;
use std::cmp::*;
 
#[fastout()]
fn main() {
    input! {
        N:usize
    }
    let mut A = vec![vec![0; 2 * N]; 2 * N];
    for i in 1..=2 * N - 1 {
        for j in i + 1..=2 * N {
            input! {x:usize}
            A[i - 1][j - 1] = x;
            A[j - 1][i - 1] = x;
        }
    }

    let ans = dfs(&A, 0, 0, 0);
    println!("{}", ans);
}
 
fn dfs(A: &Vec<Vec<usize>>, used: usize, b: usize, i: usize) -> usize {
    let N2 = A.len();
    if N2 == i { //base case　and sum of xor
        return b;
    } else {
        if used & (1 << i) != 0 { //一人目選ぶ.すでにペア組んでた場合は次に進む（次はさらに一つずらす）
            return dfs(A, used, b, i + 1);
        } else {
            let mut tmp = 0;
            for j in i + 1..N2 {
                if (1 << j) & used != 0 { // 二人目選ぶ（まだペア組んでない場合(どっちも0)はその二人を選んでペアにする）
                    continue;
                }
                tmp = max(tmp, dfs(A, used | (1 << i) | (1 << j), b ^ A[i][j], i + 1));
            } //usedが全部111...1になるまでループする。そうしながらxor演算の合計をためる。
            return tmp;
        }
    }
}
/* 
#[fastout]
fn main() {
    input! {n:usize}
    let mut A: Vec<Vec<usize>>= Vec::new();
    for i in (0..n).rev(){
        input! {a:[usize;2*i]}
        A.push(a);
    }
    
    let mut ans :usize= 0;
    let n2 = n*2;

    //A[0][0 ] is a 12
    //A[i][j] is  a i+1, i+1+j+1
    //a I,J is A[I-1][J-I-1]

    //hashmap??

for _ in 0..n2 {}
    let hs = HashMap::new();
    let result = Vec::new();
    for i in 1..n2+1 {
        for j in i..n2+1{
            let I = i -1;
            let J = j - i -1;
            if hs.contains_key(&i) || hs.contains_key(&j) {continue;}

            
            hs.insert(i,1);
            hs.insert(j,1);
        }
    }
    for vec in (0..n2).combinations(2){
        //vec[0] = i (<j)
        //vec[1] = j 

        //... result += (a[][] ^ a[][])

    }

    ans = ans.max(result);
 



    println!("{}", ans);

}

*/