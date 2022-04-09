use proconio::input;

fn val(a: u64, b: u64) -> u64 {
    return a*a*a + a*a*b + a*b*b + b*b*b;
}

fn main() {
    input! {
        n: u64,
    }

    let mut a:u64 = 0;
    let mut b: u64 = 0;
    while val(a, b) < n {
        a += 1;
    }
    let mut x = val(a, b);
    while a > b {
        a -= 1;
        while val(a, b) < n {
            b += 1;
        }
        x = if x > val(a, b) { val(a, b) } else { x };
    }
    println!("{}", x);
}
