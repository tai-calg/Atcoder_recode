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
    }//とりあえずaだけでMaxこえるまで＋＋ 計算量はMax;O(e+6)
    let mut x = val(a, b);
    while a > b {//対称性よりa > b 以降の演算をしても意味がないので。 Max;O(0.5* e+6)
        a -= 1;
        while val(a, b) < n {//山をならしていく。
            b += 1;
        }
        //minを更新
        x = std::cmp::min(x, val(a, b));
    }
    println!("{}", x);
}
