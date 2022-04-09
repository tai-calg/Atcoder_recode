
/* https://atcoder.jp/contests/abc246/tasks/abc246_e
アライグマ「E問題は、ABC170F『Pond Skater』のK=∞バージョンなのだ！
(今いる座標,向き)を頂点にして、向きを変えるときにコストがかかる01BFSだと思えばいいのだ！」
*/

use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Bytes, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: (Usize1, Usize1),
        b: (Usize1, Usize1),
        s: [Bytes; n]
    }
    let mut deq = VecDeque::new();
    let d = [(1, 1), (1, !0), (!0, !0), (!0, 1)];
    let mut dist = vec![vec![std::usize::MAX; n]; n];
    dist[a.0][a.1] = 0;
    deq.push_front(a);
    while let Some((x, y)) = deq.pop_front() {
        let cost = dist[x][y] + 1;
        for &(dx, dy) in d.iter() {
            let mut x = x + dx;
            let mut y = y + dy;
            while x < n && y < n && s[x][y] == b'.' && dist[x][y] >= cost {
                if dist[x][y] > cost {
                    dist[x][y] = cost;
                    deq.push_back((x, y));
                }
                x += dx;
                y += dy;
            }
        }
    }
    if dist[b.0][b.1] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[b.0][b.1]);
    };
}
