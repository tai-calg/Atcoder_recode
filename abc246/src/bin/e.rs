/* https://atcoder.jp/contests/abc246/tasks/abc246_e
アライグマ「E問題は、ABC170F『Pond Skater』のK=∞バージョンなのだ！
(今いる座標,向き)を頂点にして、向きを変えるときにコストがかかる01BFSだと思えばいいのだ！」
*/
use std::collections::VecDeque;

use proconio::{input, marker::Usize1};
// E
const INF: u64 = 1_000_000_000_000_000_000;
fn main() {
    input! {
        n: usize,
        a: (Usize1, Usize1),//start
        b: (Usize1, Usize1),//goal
        mat: [String;n]
    }

    let mat = mat
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut dist = vec![vec![INF; n]; n]; // 距離！？
    let mut que = VecDeque::new();
    que.push_back((a.0, a.1));
    dist[a.0][a.1] = 0;

    while let Some((x, y)) = que.pop_front() {
        if x == b.0 && y == b.1 {
            break;
        }

        for (dx, dy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)].iter() {
            for d in 1..(n as i32) {//行けるところまで行く、段階的に行く。
                let nx = x as i32 + dx * d;
                let ny = y as i32 + dy * d;
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
                    break;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if mat[nx][ny] == '#' { //範囲外に出るOR #にぶつかるまで進む
                    break;
                }
                if dist[nx][ny] > dist[x][y] + 1 {
                    dist[nx][ny] = dist[x][y] + 1;//その地点に何回目で到達できるか。（最小値更新）
                    que.push_back((nx, ny));//更新できたらキューに入れる。
                }
                if dist[nx][ny] <= dist[x][y] {//更新できなかったのならその方向に行く価値はないのでbreak
                    break;
                }
            }
        }
    }

    if dist[b.0][b.1] == INF {
        println!("-1");
    } else {
        println!("{}", dist[b.0][b.1]);
    }
}
