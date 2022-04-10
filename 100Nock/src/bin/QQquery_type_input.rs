
// https://atcoder.jp/contests/abc247/tasks/abc247_d 
/*アライグマ「D問題はランレングス符号化するのだ！
　筒の中を「10が100個、2が50個、10が10000個、7が77個、……」
みたいに覚えておくといいのだ！」
 */
use std::collections::VecDeque;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {q: usize}
    let mut p = VecDeque::new();
    for _ in 0..q {
        input! {t: u8}
        if t == 1 {
            input! {
                x: u32,
                c: u32,
            }
            p.push_back((c,x));
        } else {
            input! {
                mut c: u32,
            }
            let mut ans = 0u64;
            while c!=0 && p[0].0 <= c {
                ans += p[0].0 as u64 * p[0].1 as u64;
                c -= p[0].0;
                p.pop_front().unwrap();
            }
            if c !=0 {
                ans += c as u64 * p[0].1 as u64;
                p[0].0 -= c;
            }
            println!("{}", ans);
        }
    }
}

/* 
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::VecDeque;
use proconio::*;
 
#[fastout]
fn main() {
    input! {q:usize}

    let mut Q : Vec<String> = Vec::new();
    for _ in 0..q {
        let x = read();
        Q.push(x);
    }
    assert!(Q.len() == q);

    let mut deq :VecDeque<usize> = VecDeque::new();

    for query in Q {
        let query = query.chars().collect::<Vec<char>>();

        if query[0] == '1' { //push_front
            for _ in 0..(query[4] as usize - 48) {
                deq.push_front(query[2] as usize - 48) ;
            }
            
        }else { //pop_back
            let mut sum = 0;

            for _ in 0..(query[2] as usize - 48) {

                if let Some(x) = deq.pop_back() {
                    sum += x;
                }else {

                }
            }

            println!("{}", sum);
        }


    }

}

fn read() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().parse().unwrap()
}

*/