use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashMap, HashSet};
use std::cmp::max;
use std::usize::MAX;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
        b: [usize; n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }
    let mut ca = vec![0; n];
    let mut map = HashMap::new();
    for i in 0..n {
        if !map.contains_key(&a[i]) {
            map.insert(a[i], map.len() + 1);
        }
        ca[i] = map.len();
    }
    let mut set = HashSet::new();
    let mut cb = vec![0; n];
    let mut mb = vec![0; n];
    let mut m = 0;
    for i in 0..n {
        set.insert(b[i]);
        cb[i] = set.len();
        let v = *map.get(&b[i]).unwrap_or(&MAX);
        m = max(m, v);
        mb[i] = m;
    }
    for (x, y) in xy {
        if ca[x] == cb[y] && cb[y] == mb[y] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}