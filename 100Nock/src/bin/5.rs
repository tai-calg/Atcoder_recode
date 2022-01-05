/*
ファーストフードチェーン「ピザアット」のメニューは「A ピザ」「B ピザ」「AB ピザ」の 3 種類です
。A ピザと B ピザは全く異なるピザで、これらをそれぞれ半分に切って組み合わせたものが AB ピザです。
A ピザ、B ピザ、AB ピザ 1 枚あたりの値段はそれぞれ A 円、B 円、C 円です。

中橋くんは、今夜のパーティーのために A ピザ X 枚と B ピザ Y 枚を用意する必要があります。
これらのピザを入手する方法は、A ピザや B ピザを直接買うか、AB ピザ 2 枚を買って A ピザ 1 枚と B ピザ 1 枚に組み替える以外にはありません。このためには最小で何円が必要でしょうか？
なお、ピザの組み替えにより、必要な量を超えたピザが発生しても構いません。
*/
use proconio::{input, fastout};

fn main (){
    input! {mut a:usize, mut b:usize,c:usize,x:usize,y:usize};

    let bigger;
    let diff ;
    let mut  biggerx :bool = false;
    if x>y { bigger = x; diff = x -y; biggerx = true}else{bigger = y; diff = y-x;};


    let ans1 = (bigger-diff) * cheap_pizza(a, b, c, false) ;
    if biggerx {b = 0;}else{a = 0;};
    
    
    let ans2 =  diff * cheap_pizza(a, b, c, true);

    println!("{}", ans1 + ans2);
}

fn cheap_pizza(a: usize, b:usize, c:usize, isleft: bool) -> usize{
    if !isleft {
        return (c*2).min(a+b);
    }else {
        //残り０枚の方は０にしておく
        let exsit = a.max(b);
        return (c*2).min(exsit);
    }
}

/* ANSWER
    let w = min(x, y);
    let v = max(x, y);
    let set = w * min(a + b, c * 2);
    if x > y {
        println!("{}", min(set + (x - y) * a, v * 2 * c));
    } else {
        println!("{}", min(set + (y - x) * b, v * 2 * c));
    }

*/