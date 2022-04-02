#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
 
#[fastout]
fn main() {
    input! {n:isize, k:isize, x:isize,mut A: [isize;n]}

        A.sort();
        A.reverse();

    
        let k = coupon(&mut A, k , x, false);



    A.sort();
    A.reverse();

    let _k = coupon(&mut A, k , x, true);



    println!("{}",A.iter().sum::<isize>());






}


fn coupon(vec: &mut Vec<isize>, k:isize, x:isize, issecond:bool) -> isize{ //残りのkを返す。vecはソートしておけ
    let mut count = 0;

    if issecond == false {
        for i in 0..vec.len() {
            while (vec[i].clone()  - x ) > 0{
                vec[i] = std::cmp::max(0,vec[i] - x) ;
                count +=1;
                if count == k {
                    return 0;
                }
            }
    
        }
    
        return k - count;

    }else {
        
        for i in 0..vec.len() {
            if count == k {
                return 0;
            }
                vec[i] = std::cmp::max(0,vec[i] - x) ;

                count +=1;
    
        }

    }

    return k;

}