#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {n:usize, T:String}

    let t = T.chars().collect::<Vec<char>>();

    let mut xy = (0,0);
    let mut count = 0;

    for &i in t.iter(){
        if i == 'S' {
            modfn(count, &mut xy);

        }else if i == 'R' {
            count += 1;
        }

    }

    println!("{} {}", xy.0, xy.1);

}


fn modfn(count:i32,  xy: &mut (i32,i32)) {
    if count % 4 == 0 {
        xy.0 += 1;
    }else if count % 4 == 1 {
        xy.1 -= 1;
    }else if count % 4 == 2 {
        xy.0 -= 1;
    }else if count % 4 == 3 {
        xy.1 += 1;
    }

}
