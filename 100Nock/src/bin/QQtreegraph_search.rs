use std::collections::VecDeque;
use proconio::{input};
// https://atcoder.jp/contests/abc138/tasks/abc138_d 
// 木も所詮グラフなのでグラフを作って処理する
fn main() {
    input!{n:usize,q:usize,abn:[(usize,usize);n-1],pxq:[(usize,usize);q]}
    let mut graph = vec![vec![];n];
    let mut ans = vec![0;n];
    let mut used = vec![false;n];
    for (a,b) in abn{
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    for (p,x) in pxq{
        ans[p-1] += x;
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    used[0] = true;
    while let Some(v) = queue.pop_front(){
        for next in &graph[v]{
            if *next == v || used[*next]{
                continue;
            }
            ans[*next] += ans[v];
            used[*next] = true;
            queue.push_back(*next);
        }
    }
    println!("{}",ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))
}

