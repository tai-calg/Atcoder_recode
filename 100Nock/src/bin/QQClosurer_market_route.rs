/*
AtCoder マーケットは、1 000 000 000 個のマスが 1 列につながったマス目で表されるスーパーマーケットである。
ここでは、左から i 番目のマスを「マス i」とする。

ある日、N 人の買い物客が AtCoder マーケットに来る。i 人目の買い物客は、マス Ai​にある品物とマス Bi
​にある品物を買う。

square1001 君は、AtCoder マーケットに入口と出口を 1 つずつ設置することにした。
入口と出口はいずれかのマス目に設置する。入口と出口は同じ場所にあってもよい。

そのとき、買い物客は次のような経路で移動する。

まず、入口からスタートする。マス Aiと Bi
​を経由して、出口でゴールする。
すべての買い物客について、隣り合ったマス目に進むのに 1 秒かかるとき、最適に入口と出口を設置したときの
「すべての買い物客の移動時間の合計」の最小値を求めなさい。



 1 ≤ 𝑁 ≤ 30
  1 ≤ Ai < Bi ≤ 10^9 で整数
  use proconio::{input, fastout};
  fn main() {
      
}
*/

/*  answer 解説 : https://img.atcoder.jp/s8pc-6/editorial.pdf*/

fn main() {
    let n: usize = read();
    let mut res = 0;
    let mut abv = vec![];
    for _ in 0..n {
        let (mut a, mut b): (i64, i64) = (read(), read());
        abv.push((a, b));
        res += b - a;
    }
    let mut calcmidist = |i: usize| -> i64 {
        abv.sort_by(|x, y| x.0.cmp(&y.0));
        let abase = abv[i].0;
        abv.sort_by(|x, y| x.1.cmp(&y.1));
        let bbase = abv[i].1;
        /// __デリゲートって処理の中に具体的な変数を入れ込む&定義することができるのか（ここではabv）
        // n/2
        let rem: i64 = abv
            .iter()
            .map(|ab| (ab.0 - abase).abs() + (ab.1 - bbase).abs())
            .sum();
        rem
    };
    if n % 2 == 0 {
        // n/2 or n/2 - 1
        res += min(calcmidist(n / 2), calcmidist(n / 2 - 1));
    } else {
        res += calcmidist(n / 2);
    }
    println!("{}", res);
}
/// 前提として、AiBiそれぞれの中央値のマスに出入り口を置くのが最適解だと、例題をみて決めうちしている解法である
/// sigma|Ai - Bi| + sigma(|Ai - Amid| + |Bi - Bmid|)
/// 他の人の回答をみても、入口がa, 出口がbの中にあると仮定してforforしてる回答がほとんど
