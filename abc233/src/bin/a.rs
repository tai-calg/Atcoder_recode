use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {x:isize, y:isize};

    let mut  i =  y - x;
    if i % 10 != 0 { 
        i = i /10;
        i += 1;
    }else {
        i =i /10;
    }

    if y-x > 0{
        println!("{}",i);
    }
    else{
        println!("{}",0);
    }
}
