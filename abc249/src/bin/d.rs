#[allow(unused_imports)]
use proconio::{input,marker::*};
//同じ数があるときに対する処理を思いつけるかどうかの問題
fn main() {
    input!{
        n: usize,
        aa: [usize; n],
    }
    let mx=*aa.iter().max().unwrap();
    // let mx=200000;
    let mut counter = vec![0; mx+1];
    for a in aa {
        counter[a] += 1;
    }

    
    let mut ans = 0usize;
    
    for ak in 1..=mx {
        for aj in 1..=mx/ak{
            // if counter[ak]*counter[aj]*counter[aj*ak]>0{
            //     eprintln!("{}:{} {}:{} {}:{}",aj*ak,counter[aj*ak],aj,counter[aj],ak,counter[ak]);
            // }
            ans += counter[ak]*counter[aj]*counter[aj*ak];
        }
    }
    println!("{}",ans);
}
