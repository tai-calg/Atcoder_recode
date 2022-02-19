fn main() {
    proconio::input! {
        n: usize,
        x: u64,
        bags: [[u64]; n],
    }
 
    let count = calc(&bags, 0, x);
    println!("{}", count)
}

//別解
fn calc(a: &Vec<Vec<u64>>, n: usize, x: u128, i: usize, v: u128) -> u32 {
    if i >= n {
        if v == x {1} else {0}
    } else {
        let mut c = 0;
        for j in 0..a[i].len() {
            c += calc(a, n, x, i + 1, v * a[i][j] as u128)
        }
        c
    }
}