use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {a: i16};
    input! {b: i16, c: i16};
    input!{s:String};
    
    println!("{}", a + b + c);
    println!("{}", s);

}
