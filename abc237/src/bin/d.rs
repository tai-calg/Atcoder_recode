use std::collections::VecDeque;
 
use itertools::Itertools;
use proconio::{input, marker::Chars};


//時を戻そう(reverse)
fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut q = VecDeque::new();
    q.push_back(n);
    for i in (0..n).rev() {
        if s[i] == 'L' {
            q.push_back(i);
        } else {
            q.push_front(i);
        }
    }
    println!("{}", q.iter().join(" "));
}

    //time out , insert 部分の処理を自分で工夫して実装しないとダメ


/*
    for (i,s) in enumerate(s_) {

        if s == 'L' {
            A.insert(insPos, i+1);

        }else{ //R
            A.insert(insPos+1, i+1);
            insPos +=1;
            
        }
    }

        for a in A {
        print!("{} ",a);
    }
*/