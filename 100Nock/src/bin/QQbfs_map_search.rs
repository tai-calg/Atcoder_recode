use proconio::input;
use std::collections::VecDeque;
use proconio::marker::Chars;
const INF : u32 = 10_000;
const DX : [isize;4]= [-1,0,1,0];
const DY : [isize;4]= [0,-1,0,1];
fn main() {
    input! {
        r: usize, 
        c: usize,
        sts: [usize;2],
        gls: [usize;2],
        mazes: [Chars;r]
    };
    let mut memos = vec![vec![INF;c];r];
    memos[sts[0]-1][sts[1]-1] = 0;
    let mut q = VecDeque::new();
    q.push_back((sts[0]-1, sts[1]-1));
    while let Some(t) = q.pop_front() {
            for i in 0..4 {
                let ny = (t.0 as isize + DY[i]) as usize;
                let nx = (t.1 as isize + DX[i]) as usize;
                if mazes[ny][nx] != '#' { 
                    let c = memos[t.0][t.1] + 1;
                    if memos[ny][nx] > c {
                        memos[ny][nx] = c; 
                        q.push_back((ny,nx));
                    } 
                }
            }
        }
    println!("{}", memos[gls[0]-1][gls[1]-1]);
}
