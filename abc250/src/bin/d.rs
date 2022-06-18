use proconio::input;

fn gen_prime_list(n: usize) -> Vec<usize> {
    let mut ret = vec![];
    let mut is_prime = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 0..=n {
        if !is_prime[i] {
            continue;
        }
        ret.push(i);
        let mut j = i + i;
        while j <= n {
            is_prime[j] = false;
            j += i;
        }
    }
    ret
}

fn f(p: usize, q: usize) -> u128 {
    let p = p as u128;
    let q = q as u128;
    p * q * q * q
}

fn main() {
    input! { n: u128 }
    let a = gen_prime_list(1_000_000);
    let m = a.len();
    let mut j = m - 1;
    let mut ans = 0;
    for i in 0..m {
        while i < j && f(a[i], a[j]) > n {
            j -= 1;
        }
        if i < j {
            ans += j - i
        }
    }
    println!("{}", ans);
}