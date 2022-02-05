
use proconio::input;
//N(16程度)人で徒競走をする、
//xさんとyさんの順序関係がM個与えられるから、矛盾しない順位の数を計算しなさい。

fn main() {
    input! {
        N: usize, M: usize,
        XY: [(usize, usize); M],
    }

    let mut lates = vec![vec![]; N];
    for (x,y) in XY {
        lates[x-1].push(y-1);
    }

    let mut dp = vec![None; 1<<N];
    dp[(1<<N)-1] = Some(1);

    fn rec(s: usize, N: usize, lates: &Vec<Vec<usize>>, dp: &mut Vec<Option<usize>>) -> usize {
        if dp[s].is_some() {
            return dp[s].unwrap()
        }
        let mut res = 0;
        for x in 0..N { // N
            if s & (1<<x) > 0 {
                continue;
            }
            for y in &lates[x] {
                if s & (1<<*y) > 0 {
                    dp[s] = Some(0);
                    return 0
                }
            }
            res += rec(s | (1<<x), N, lates, dp);
        }
        dp[s] = Some(res);
        return res;
    }

    println!("{}", rec(0, N, &lates, &mut dp));
}