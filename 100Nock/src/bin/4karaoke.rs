/*
1,2,...,N と番号づけられている N 人の生徒から成るグループが，「全国統一カラオケコンテスト」に出場することとなりました．

このコンテストで歌える曲は，曲 1，曲 2，...，曲 M の M 曲あります．また，番号 i の生徒が曲 j を歌うと，必ず A 
i,j
​
  点を取ります．

さて，コンテストのルールは，以下のようになります．

M 曲の中から 2 つの曲を選ぶ．

各生徒の得点は，その生徒が歌った 2 つの曲の点数のうち高い方となる．
グループの得点は，生徒 1,2,...,N の得点の合計となる．
そのとき，グループの得点として考えられる最大の値を求めてください．

1≤N≤100
2≤M≤100
0≤Ai,j≤100 000 000
入力はすべて整数
*/

use proconio::{input, fastout};

fn main (){
    input!{n: usize, m:usize};
    input!{A: [[usize;m] ; n]};


    
    let mut max = 0;
    for i in 0..m{
        for j in i..m {
            let mut sum = 0;
            for k in 0..n{ //kは人
                let personalMax = A[k][i].max(A[k][j]);
                sum += personalMax;

            }
            if sum > max {
                max = sum;
            }
        }
    }

    println!("{}", max);
    //n,m <= 100だから三十ループでもなんとか許された
}