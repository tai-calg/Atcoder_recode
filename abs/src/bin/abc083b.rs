use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        mut n: u128,
        a: u128,
        b: u128,
    }


    let mut all_sum = 0;
    for i in 1..n+1 {
        let mut j = i;
        let mut isum : u128 = 0;

        
        while j !=0 {
            let amari = (j % 10) as u128;
            j = j /10;
            isum += amari;

        }

        if a <= isum   && isum <= b {
            all_sum += i;
        }

    }

    println!("{}", all_sum);





}
