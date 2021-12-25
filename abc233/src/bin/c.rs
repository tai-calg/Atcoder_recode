
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
