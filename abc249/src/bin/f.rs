use std::cmp::Reverse;
use std::collections::BinaryHeap;
use proconio::input;

fn main() {
    input! {
        n : usize,
        mut k : usize,
        mut queries : [(u8, i64); n]
    }

    queries.reverse();
    queries.push((1, 0));

    let mut ans = std::i64::MIN;

    let mut p = 0;
    let mut pq = BinaryHeap::new();
    for (t, y) in queries {
        match t {
            1 => {
                ans = ans.max(y + p);
                if k <= 0 { break }
                k -= 1;
            }
            2 => {
                if y < 0 { pq.push(y); }
                else { p += y; }
            }
            _ => unreachable!()
        }
        while pq.len() > k { if let Some(m) = pq.pop() { p += m; } }
    }

    println!("{}", ans)
}