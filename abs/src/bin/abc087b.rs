

use proconio::{input, fastout};


#[fastout]
fn main() {


    let mut count = 0;

    

    input! {a:i128};
    input! {b:i128};
    input! {c:i128};
    input! {x:i128};

    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                if i*500 + j*100 + k*50 == x {
                    count += 1;

                }
            }
        }
    }

        
    


    
    println!("{}",count);

}
