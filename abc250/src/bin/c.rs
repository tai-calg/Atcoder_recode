
//https://atcoder.jp/contests/abc250/tasks/abc250_c 

use proconio::input;
use proconio::marker::*;

fn solve1() {
    input!{
        n: usize,
        q: usize,
        xs: [Usize1; q],
    }
    let mut pos = (0..n).collect::<Vec<usize>>();
    let mut val = pos.clone();
    for x in xs {
        let p1 = pos[x];
        let p2 = if p1 == n - 1 { p1 - 1 } else { p1 + 1 };
        let v1 = val[p1];
        let v2 = val[p2];
        pos.swap(v1, v2);
        val.swap(p1, p2);
    }
    let ans = val.iter()
        .map(|v| (v+1).to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);

}
fn main() {
    solve1();
}
/* 

fn main() {
    input! {
        n:usize , q:usize,
        X:[usize;q],
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
        let mut neighbor = 0;
        if idx <n {
            neighbor = a[idx+1];
            let ni = idx+1;
            swap(&mut a,idx,ni);
            *hm.entry(x).or_insert(0) +=1;
            *hm.entry(neighbor).or_insert(0) -=1;

        }else {
            neighbor = a[idx-1];
            let ni = idx-1;
            swap(&mut a,idx,ni);
            *hm.entry(x).or_insert(0) -=1;
            *hm.entry(neighbor).or_insert(0) +=1;
        }


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
*/