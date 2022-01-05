
//
//use proconio::{input, fastout};
//
//fn main (){
//    input! {n:usize};
//    input! {s:usize};
//
//    let st:Vec<char> = s.to_string().chars().collect();
//
//    for i in 0..st.len(){
//        for j in i+1..st.len(){
//            //let mut tmpst = &st.clone()[j+1..st.len()];
//            //tmp.
//        }
//    }
//}

// ANSWER
use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let s = s.as_bytes().iter().map(|u| u - b'0').collect::<Vec<_>>();
    let mut answer = 0;
    for i in 0..=9 {
        let pi = s.iter().position(|&u| u == i); //左から条件を満たしたものを探してそのインデックスを返す
        for j in 0..=9 {
            let pj = s.iter().rposition(|&u| u == j); //右から～。　この後.nextするとそのインデックスの左隣が選択される
            match (pi, pj) {
                (Some(lo), Some(hi)) if lo < hi => {
                    let mut d = [false; 10];
                    for k in lo + 1..hi {
                        d[s[k] as usize] = true;
                    }
                    answer += d.iter().filter(|&b| *b).count();
                }
                _ => {}
            }
        }
    }
    println!("{}", answer);
}
//
//どうしても三十ループにはなるが、ループさせる個数を１００００とかにさせないのが大事