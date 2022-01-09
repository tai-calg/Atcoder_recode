
/*
英大文字からなる文字列 S が与えられます。S の部分文字列 (注記を参照) であるような
最も長い ACGT 文字列 の長さを求めてください。

ここで、ACGT 文字列とは A, C, G, T 以外の文字を含まない文字列です。
*/

use std::char;

use proconio::{input, fastout};

fn main() {
    input! {s:String};

    let st:Vec<char> = s.chars().collect();
    let mut count = 0;
    let mut ans = 0;
    for c in st {
        if is_acgt(c){
            count += 1;
            ans = ans.max(count);
        }else{
            count = 0;
        }
    }

    println!("{}", ans);

}

fn is_acgt(c:char) -> bool {
    match c {
        'A' |'C' |'G' |'T' =>  true,
        _ => false
    }
}