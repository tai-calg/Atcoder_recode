use std::io;
 
fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().split(' ').flat_map(str::parse).collect()
}
fn main() {
    //input! {
    //    n:usize,
    //    a:[usize;n],
    //    q:usize,
    //    m:[usize;q]
    //};
    let n :usize= read::<usize>()[0];
    let a = read::<usize>();
    let q = read::<usize>()[0];
    let m = read::<usize>(); //AOJ提出用の書き方


    for &mi in &m {
        if dfs(0,&a,0,mi) {
            println!("yes");
        }
        else {
            println!("no");
        }
    }
}

fn dfs(i:usize, A:&Vec<usize>, sum:usize ,goal:usize)->bool{
    if i ==A.len() { return sum == goal; }

    if (dfs(i+1, &A, sum,goal)) {return true;}

    if dfs(i+1, &A, sum+ &A[i],goal) {return true;}

    return false;

}