use std::collections::HashSet;
use std::cmp::max;
use proconio::{input, fastout};
//extern crate num;
//use num::complex::Complex;
fn main() {
    //let x = Complex::new(1,1);
    //let y = Complex::new(2,2);
    //let i = Complex::new(0,1);
    //let z = (y - x ) * i + x;
    //println!("{}",z);


    input!{n:usize};
    input! {points:[(i64,i64);n]};
//
    //let i = Complex::new(0,1);
    //let mut edges:Vec<usize> = Vec::new();
//
    //for i in 0..n-1{
    //    for j in i+1..n{
    //        let length = (Complex::new(points[i][0], points[i][1]) 
    //            - Complex::new(points[j][0], points[j][1])).norm_sqr() ; 
    //        edges.push(length);
    //        
    //    }
    //}
    //println!("{:?}",edges);

    let ans = solve(n, &points);
    println!("{}",ans);



}

fn solve(_: usize, points: &[(i64,i64)])->i64 {
    let hs:HashSet<(i64,i64)> = points.iter().cloned().collect();
    let mut ret = 0;

    for &(x1,y1) in points {
        for &(x2,y2) in points {
            if x1 == x2 && y1 == y2 { continue; } //二つのfor が同じ場所を指すようにしてるので重複を無視するように
            
            let (ax , ay ) = ((x2 - x1) , (y2-y1));
            let (x3 , y3 ) = (x1 + ay , y1 - ax);
            let (x4 , y4 ) = (x2 + ay , y2 - ax);

            if hs.contains(&(x3,y3)) && hs.contains(&(x4,y4)) {
                let sq = ax* ax + ay * ay;
                ret = max(ret , sq);
            }

        }
    }

    return ret;
}

/*
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
 
    let n: i64 = sc.input();
    let arr: Vec<(i64, i64)> = (0..n)
        .map(|_| (sc.input(), sc.input()))
        .collect();
 
    let mut is_pillar = vec![vec![false; 5001]; 5001]; //ここから
    for g in &arr {
        let (x, y) = *g;
        is_pillar[x as usize][y as usize] = true;
    }// 座標を作って柱があるところをプロット
 
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = arr[i as usize];
            let (x2, y2) = arr[j as usize];
 
            let square = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
            if square <= ans {
                continue;
            }
 
            let x3 = x2 + y2 - y1;
            let y3 = y2 + x1 - x2;
            if x3 < 0 || x3 > 5000
                || y3 < 0 || y3 > 5000 || is_pillar[x3 as usize][y3 as usize] == false{
                continue;
            }
            let x4 = x1 + y2 - y1;
            let y4 = y1 + x1 - x2;
            if x4 >= 0 && x4 <= 5000
                && y4 >= 0 && y4 <= 5000 && is_pillar[x4 as usize][y4 as usize]  {
                ans = square;
            }
        }
    }
    println!("{}", ans);
}
*/