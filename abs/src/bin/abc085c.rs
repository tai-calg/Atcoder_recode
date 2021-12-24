use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {mut n:isize, mut y:isize};

    let mut num:[isize;3] = [-1,-1,-1];

    for i in 0..n+1{
        for j in 0..n+1{
            let total = 10000*i + 5000*j + 1000*(n-i-j);
            if total == y  && n-j-i >=0{
                num = [i,j,n-i-j];
            }

            }
        }
    

    println!("{} {} {}",num[0],num[1],num[2] );



}

/*貪欲法でやって失敗 == の組み合わせ問題には貪欲法は適さない
    //let mut result = 0;
    let mut num:[usize;3] = [0,0,0];
    for i in 0..3{
        num[i] = y / VALUE[i];

        y -= VALUE[i] * num[i];
        n -= num[i] as isize;
    }

    if y == 0 && n == 0 {
        println!("{} {} {}",num[0],num[1],num[2] );
    }else{
        println!("{} {} {}", -1 , -1 ,-1 );
    }

*/