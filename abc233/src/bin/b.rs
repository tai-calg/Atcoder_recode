use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {l:usize, r:usize};
    input! {mut s:String};


    //s = s.chars().collect();
    //let fst = &s[0..l]; 
    //let aft = &s[r..]; 
    //let mut inv  = &s[l-1..r];
    //inv = (*inv).chars().rev().collect::<&str>();

    let (fst , mid ) = s.split_at(l-1);
    let (inv, aft) = mid.split_at(r-l+1);
    let inved = inv.to_string().chars().rev().collect::<String>();
    println!("{}{}{}",fst.to_string() , inved , aft.to_string());

    //s.insert_str
}
