
use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n:usize, mut x:usize};

    let mut dvec:Vec<Vec<usize>> = Vec::new();
    let mut num_vec :Vec<usize> = Vec::new();
    for _ in 0..n {
        input! {num : usize, v:[usize;num]};
        //let invec :Vec<usize>= num.to_string().split_whitespace().map(|c| c.parse().unwrap()).collect();
        num_vec.push(num);
        dvec.push(v);
    }


    

    //let kk = dvec[1].clone() ;
    let mut result = x;
    //for (i ,ivec) in dvec.iter().enumerate() {
    //    if x % 
//
    //}

    //for i in 0..dvec.len(){
    //    for jhai in &dvec {
    //        for j in 0..jhai.len(){
    //            if x % jhai[j] == 0 {
    //                result /= jhai[j];
    //                break;
    //            }
    //        }
//
    //    }
    //}

    for jvec in &dvec{
        for j in jvec{

        }
    }
    

}

/* ANSWER
fn count_patterns(bags: &Vec<Vec<u64>>, index: usize, x: u64) -> usize {
    let bag = &bags[index];
 
    if index == bags.len() - 1 {
        bag
            .iter()
            .filter(|item| **item == x)
            .count()
    } else {
        bag
            .iter()
            .map(|item| {
                if x % item != 0 {
                    0
                } else {
                    count_patterns(bags, index + 1, x / item)
                }
            })
            .sum()
    }
}
 
fn main() {
    proconio::input! {
        n: usize,
        x: u64,
        bags: [[u64]; n],
    }
 
    let count = count_patterns(&bags, 0, x);
    println!("{}", count)
}

//別解
fn calc(a: &Vec<Vec<u64>>, n: usize, x: u128, i: usize, v: u128) -> u32 {
    if i >= n {
        if v == x {1} else {0}
    } else {
        let mut c = 0;
        for j in 0..a[i].len() {
            c += calc(a, n, x, i + 1, v * a[i][j] as u128)
        }
        c
    }
}
*/
