use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        i:usize, mut S:String
    }
    S = S.chars().collect::<String>();
    println!("{}",S.pop().unwrap());
}
