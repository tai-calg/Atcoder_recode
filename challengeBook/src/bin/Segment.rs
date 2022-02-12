#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::iter::Copied;

use proconio::*;
 

struct SegumentTree<F,T> {
    tree:Vec<T>,
    size: usize,
    elem: T,
    eval:F,
}

impl<F:Fn(T,T) -> T, T:Copy + Eq + std::fmt::Debug> SegumentTree<F,T> {
    fn new(max:usize, elem:T, eval: F)-> Self {
        let size = max.next_power_of_two();

        Self {
            size,
            tree:vec![elem;size*2],
            elem,
            eval //判定式の内容もインスタンス化の時に自由に決められる
        }
    }

    fn get(&self, left:usize, right:usize)->T{
        return self._get(left,right+1 , 1, 0, self.size);
    }

    fn _get(&self, left:usize, right:usize, now_pos: usize, l:usize, r:usize)-> T {
        //探索範囲を超えた
        if r<= left || l >= right {
            self.elem
        }
        else if left <= l && r <= right { // [a,b)が[1, r)を完全に含んでいれば、この節点の値
            self.tree[now_pos]
        }
        else {//探索が続く場合
            //子ノードに移動
            //左の子へなら、右端をずらす
            //右の子へなら、左端をずらす
            //関数施行
            (self.eval)(self._get(left, right, now_pos * 2, l, (l+r)/2 ), 
                        self._get(left, right, now_pos * 2 + 1, (l+r)/2, r),)
        }
    }

    pub fn update(&mut self, index:usize, value: T){
        let mut i = self.size + index;
        while i != 0 {
            let before = self.tree[i];
            let after = (self.eval)(before, value);

            if before == after {
                break;
            }

            self.tree[i] = after;
            i /= 2;
            
        }
    }
}

impl<F,T:std::fmt::Debug> std::fmt::Debug for SegumentTree<F,T> {
    fn fmt(&self, f:&mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        write!(f, "SegmentTree{:?}", self.tree)
    }
}


//AtCoder Beginner Contest 185-F
fn main (){
    input! {n:usize, q:usize};
    let mut seg_tree = SegumentTree::new(n,0,|a,b| a ^ b);
    input! {aVec: [usize;n]};
    for (i,v) in aVec.iter().enumerate() {
        seg_tree.update(i, *v);
    }

    //println!("{:?}", seg_tree);

    for _ in 0..q {
        input! {t:usize, x:usize, y:isize};

        match t == 1 {
            true => seg_tree.update(x-1, y as usize),
            false => println!("{}", seg_tree.get(x-1, (y-1) as usize)),
        }
    }
}