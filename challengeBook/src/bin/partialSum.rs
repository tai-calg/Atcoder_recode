/*
整数a1、a2、.…、anが与えられます。その中からいくつか選び、その和をちょうどkにすることができるか
を判定しなさい。

*/
use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n:usize}
    input!{a:[usize;n]}
    input!{k:usize}


    if dfs(0, 0, k, &a) {println!("{}", "Yes");}
    else {
        println!("{}", "No");
    }

}

//start from dfs(0,0) to use.
fn dfs (i:usize, sum : usize, k:usize,a: &[usize])-> bool{
    if i == a.len() { return sum == k;}

    //use a[i]
    if dfs(i+1, sum + a[i] , k, a) {return true;}

    // do not use a[i]
    if dfs(i+1, sum , k ,a) {return true;}

    return false;


}