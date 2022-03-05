#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use proconio::*;
use proconio::marker::Usize1;
use std::collections::HashMap; 


#[fastout]
fn main() {

    let cons =  998244353;

    input! {n:usize}

    let mut dp:HashMap<(usize,usize),usize> = HashMap::new(); 
    //dfsの中でdpは参照するだけにしないとダメ。
    
    let mut ans =0;

    //make dp here
    for i in 1..=9{
        dp.insert((i,0), 1);
    }

    //++でdpを作る
    //count は1,2,...,nが入る
    for count in 1..n {
        for i in 1..=9{
            let dp_key = (i,count);
            
            //if a == 1 {
            //    return (*dp).insert((a,count),dfs(a+1,count-1, dp)+dfs(a,count-1, dp)).unwrap();
            //}
            if i == 1 {
                let mut val = (dp[&(i,count-1)] + dp[&(i+1,count-1)] );
                if val > cons{
                    val %= cons;
                } 

                dp.insert(dp_key, val);
            }else if i == 9{
                let mut val = (dp[&(i,count-1)] + dp[&(i-1,count-1)] );
                if val > cons{
                    val %= cons;
                } 

                dp.insert(dp_key, val);
            }else{
                let mut val = (dp[&(i,count-1)] + dp[&(i-1,count-1)] + dp[&(i+1,count-1)]);
                if val > cons{
                    val %= cons;
                } 

                dp.insert(dp_key,  val);
            }

        }


    }

    for i in 1..=9{
        ans += dp[&(i,n-1)];
    }

    println!("{}", ans % cons);
}

/* 
fn dfs (a:usize,count:usize, dp:& HashMap<(usize,usize),usize>)->usize{
    
    if count == 0 {
        return 1;
    }

    let a_ = a  as isize;
    let c_  = count as isize;

    if dp.get(&(a,count+1)) != None {
        return *dp.get(&(a,count)).unwrap();
    }



    //if a == 1 {
    //    return (*dp).insert((a,count),dfs(a+1,count-1, dp)+dfs(a,count-1, dp)).unwrap();
    //}else if a == 9 {
    //    return (*dp).insert((a,count),dfs(a-1,count-1, dp)+dfs(a,count-1, dp)).unwrap();
    //}else{
//
    //    return (*dp).insert((a,count),dfs(a+1,count-1, dp)+dfs(a-1,count-1, dp)+dfs(a,count-1, dp)).unwrap();
    //}

    1
}

*/