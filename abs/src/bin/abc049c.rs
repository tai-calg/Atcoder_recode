use proconio::{input, fastout};

#[fastout]
fn main() {

    input! {mut s:String};
    s = s.chars().rev().collect::<String>(); //文字列を逆にする

    //vec str をvec Stringに
    let T :Vec<String>=  vec![ "dream" ,"dreamer", "erase" ,"eraser"]
    .iter().map(|s| s.chars().rev().collect::<String>()).collect();

    let t_len = vec![5,7,5,6];

    while !s.is_empty() {
        
        let mut exist = false;
        for (i,t) in T.iter().enumerate(){

            if s.len() <= 7{ //out of index のエラーハンドリング
                if s[0..] == *t{
                    s = s.split_off(t_len[i]);
                    exist = true;
                    break;
                }
            }else{
                if s[0..t_len[i]] == *t{
                    s = s.split_off(t_len[i]);
                    exist = true;
                    break;
                }
            }

        }
        if !exist{
            break;
        }
    }

    if s.is_empty(){
        println!("{}","YES");
    }else {
        println!("{}","NO");
    }

}
 
/* answer
use proconio::{fastout, input};

#[fastout]
fn main() {
    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    input!(s: String);
    let s: Vec<char> = s.chars().rev().collect();
    let mut s = &s[..];
    let mut succeeded = true;
    while s.len() > 0 {
        let matched = patterns.iter().find(|&p| s.starts_with(p));
        if let Some(p) = matched {
            s = &s[p.len()..];
        } else {
            succeeded = false;
            break;
        }
    }
    println!("{}", if succeeded { "YES" } else { "NO" });
}

*/