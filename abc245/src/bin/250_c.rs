use proconio::input;
use std::collections::HashMap;
//https://atcoder.jp/contests/abc250/tasks/abc250_c 
fn main() {
    input! {
        n:usize , q:usize,
        X:[usize;q+1],
    }
//hash
    let mut hm  = HashMap::new();
    let mut a = vec![0;n+1];
    for i in 1..=n {
        a[i] = i;
    }

    for i in 1..=q {
        hm.insert(a[i],i);
    }

    for x in X {
        let idx = *hm.get(&x).unwrap();
        let neighbor = a[idx+1];
        swap(&mut a,idx,neighbor);

        *hm.entry(x).or_insert(0) +=1;
        *hm.entry(neighbor).or_insert(0) -=1;
    }

    for i in 1..=n {
        println!("{}",a[i]);
    }

}

fn swap(a: &mut Vec<usize>, i: usize, j: usize) {
    let tmp = a[i];
    a[i] = a[j];
    a[j] = tmp;
}