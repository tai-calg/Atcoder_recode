use proconio::{input, fastout};



#[fastout]
fn main() {
    input! {s:u8};
    let mut count =0;

    s.to_string().chars().for_each(|c| {
        if c == '1' {
            count += 1;
        }
    });

    println!("{}", count);

}
