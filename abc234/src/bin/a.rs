use proconio::input;
fn main() {

    input!{t:usize};

    let ans = f(f(f(t)+t) + f(f(t)));
    println!("{}", ans);
}
fn f(x:usize)->usize{
    return x*x + 2*x + 3;
}