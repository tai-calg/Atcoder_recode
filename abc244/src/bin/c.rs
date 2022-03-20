
use std::{collections::HashSet, io, process::exit};

fn main() {
    fn read() -> usize {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        buf.trim().parse().unwrap()
    }

    let n = read();
    let mut v = (1..=((2 * n) + 1)).collect::<HashSet<_>>();

    loop {
        let tn = v.iter().next().unwrap().to_owned();
        println!("{}", v.take(&tn).unwrap());
        
        let x = read();
        match x {
            0 => exit(0),
            an => v.take(&an),
        };
    }
}

/*
    let mut name = String::new();
    print!("名前を入力してください> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut name).unwrap();

    println!("こんにちは、{}さん。", name.trim());

    

#[fastout]
fn main() {
    let n:usize = readln();
    stdout().flush().ok();

    
    //1 ~ 2n+1
    let mut hs:HashSet<usize> = HashSet::new();
    for i in 1..=2*n+1 {
        hs.insert(i);
    }


    for i in 0..2*n+1 {
        //自分
        if let Some(elem) = hs.iter().next().cloned() {
            println!("{}", &elem);
            stdout().flush().ok();
            hs.remove(&elem);
        }


        //相手からの出力を受け付ける
        let t :usize = readln();
        stdout().flush().ok();

        
        if hs.remove(&t) == false {
            break;
        }
        


    }



}

fn get_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
fn readln<T>() -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug {
    get_line().parse().unwrap()
}

*/