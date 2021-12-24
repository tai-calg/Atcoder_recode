use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{n:i32};
    input! {mut ai:[i128;n]};

    let mut count = 0;
    let mut flag_odd = false;

    loop{
        for a in ai.iter_mut() {
            if *a % 2 == 0 {
                *a /= 2;
            } else {
                flag_odd = true;
            }

        }

        if !flag_odd {
            count += 1;
        }else{
            break;
        }
    }

    println!("{}", count);
}
