/*
あなたは星空の写真の中から星座を探している．写真には必ず，探したい星座と同じ形・向き・大きさの図形がちょうど一つ含まれている．ただし，写真の中には星座を構成する星以外に余分な星が写っている可能性がある．

例えば，図 1 の星座は図 2 の写真に含まれている（丸で囲んで示した）．与えられた星座の星の座標を x 方向に 2，y 方向に −3 だけ平行移動すると写真の中の位置になる．

探したい星座の形と写真に写っている星の位置が与えられたとき，星座の座標を写真の中の座標に変換するために平行移動するべき量を答えるプログラムを書け．
*/
use proconio::{input, fastout};
fn main() {
    input! {m:usize};
    input! {mut stars:[(isize,isize);m]}

    input! {n:usize};
    input! {mut map:[(isize,isize);n]}

    //stars.sort_by_key(|k| k.0);
    map.sort_by_key(|k| k.0);
    //map.sort();

    for &(mapx,mapy) in &map {
        let xdiff = mapx - stars[0].0; //とある一点からのベクトル
        let ydiff = mapy - stars[0].1;
        //この時点でanswerのxdiff,ydiffの探索範囲を絞っておく。xdiff に0,1,2,...,10000とか全部入れない。

        let mut flg = true;
        for &(x, y) in &stars{ //全部含まれているか
            if !map.contains(&(x+xdiff, y+ydiff)){ 
                flg = false;
                break
            }
        }

        if flg{
            println!("{} {}", xdiff, ydiff);
            break
        }

    }

}

/*
#[allow(unused_imports)]
use proconio::{input,marker::*};
fn main() {
    input!{
        m:usize,
        mstars:[(isize,isize);m],
        n:usize,
        mut nstars:[(isize,isize);n],
    }
    nstars.sort();
    for &(x,y) in &nstars{
        let xdiff=x-mstars[0].0;
        let ydiff=y-mstars[0].1;
        let mut flg=true;
        for &(mx,my) in &mstars{
            if nstars.binary_search(&(xdiff+mx,ydiff+my)).is_err(){
                flg=false;
                break
            }
        }
        if flg{
            println!("{} {}",xdiff,ydiff);
            break
        }
    }
}

*/
