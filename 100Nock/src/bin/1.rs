/* 
1 から n までの数の中から、重複無しで３つの数を選びそれらの
合計が x となる組み合わせの数を求めるプログラムを作成して下さい。

複数のデータセットが入力として与えられます。
各データセットでは、空白で区切られた n、x が 1 行に与えられます。

idea:
三つの数なのでfor 三回
最後のfor はx - i -j でほしい値がわかっている状態なのでfor を決めうちで無くせる。
ただx-i-jの値が範囲内にあるのが必要条件。

*/
use proconio::{input, fastout};
fn main() {
    loop {
        let mut nx = String::new();
        std::io::stdin().read_line(&mut nx).ok();
        let nx: Vec<usize> = nx.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = nx[0];
        let x = nx[1];
        if n == 0 && x == 0 { break; }
        let mut result = 0usize;
        for i in 1..n {
            for j in i+1..n {
                if i + j > x { break; }
                let k = x - i - j; //三番目の値、kを決め打ち
                if k > j && k <= n { 
                    result += 1;
                }
            }
        }
        println!("{}", result);
    }
}