use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            if (i < n - 5 && (0..6).filter(|k| s[i + k][j] == '#').count() >= 4)//縦に伸ばす
                || (j < n - 5 && (0..6).filter(|k| s[i][j + k] == '#').count() >= 4)//横に伸ばす
                || (i < n - 5
                    && j < n - 5
                    && (0..6).filter(|k| s[i + k][j + k] == '#').count() >= 4)//斜めに伸ばす
                || (i < n - 5
                    && j < n - 5
                    && (0..6).filter(|k| s[i + 5 - k][j + k] == '#').count() >= 4)//斜めに伸ばす
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

/* 
#[fastout]
fn main() {
    input! {n:usize}
    let mut u:Vec<u8> = Vec::with_capacity(n);
    u.resize_with(n, || {0});
    let mut uv :Vec<Vec<u8>>  = Vec::new();
    uv.resize_with(n, || {u.clone()});

    //===//

    let mut dp:Vec<(usize,usize)> = Vec::new();


    for i in 0..n {
        input! {S:String}
        for (j,s) in S.chars().enumerate() {
            if s == '.'{
                uv[i][j] = 0;
            }
            else if s == '#'{
                uv[i][j] = 1;
                dp.push((i,j));
            }
        }
    }

    //println!("{:?}", uv);
}

fn continuity(uv: &Vec<Vec<u8>>,ij:(usize,usize),n:usize)->bool{

    for dx in -1..=1{
        for dy in -1..=1{
            if dx == 0 && dy == 0 {continue;}
            let (i,j) = (ij.0 as i32 + dx, ij.1 as i32 + dy);
            if i < 0 || j < 0 || i as usize >= n || j as usize >= n {continue;}
            if uv[i as usize][j as usize] == 1 {return true;}
        }
  
    }
    return false;


}

fn dfs(uv: &Vec<Vec<u8>>, i:usize ,j:usize,mut count:usize,plotcount:usize,n:usize)->bool{


    //６連続であるかどうか
    if count == 6 && plotcount <= 2 {
        return true;
    }


    for dx in -1..=1{
        for dy in -1..=1{
            if dx == 0 && dy == 0 {continue;}
            let (x,y) = (i as i32 + dx, j as i32 + dy);
            if x < 0 || y < 0 || x as usize >= n || y as usize >= n {continue;}
            if uv[x as usize][y as usize] == 1 {
                dfs(uv, x as usize, y as usize, count+1, plotcount, n);
            }else {
                dfs(uv, x as usize, y as usize, count, plotcount+1, n);
            }
        }
    }




}

fn solve(uv: &Vec<Vec<u8>>,ij:(usize,usize),n:usize){
    for i in 0..n{
        for j in 0..n{
            if uv[i][j] == 1 {
                if continuity(uv,(i,j),n) {
                    dfs(uv,i,j,1);
                }
            }
        }
    }
}

*/