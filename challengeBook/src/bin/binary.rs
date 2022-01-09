use std::fmt::format;

use proconio::input;



fn main (){
    input! {dec:usize}

    //let bin = DecToBin(dec);

    let to_bin = |v:usize| -> usize { format!("{:b}",v ).to_string().parse().unwrap() };

    let bin = to_bin(dec);
    println!("{:?}", &bin);
    
}

fn DecToBin(decimal: usize)->usize {
    let mut bit: [u8;64]  = [0;64]; 
    //80くらいからもうusizeの関係でoverflowしてエラーになる。結局format!が最強なのか…
    //でも.parseがusizeなのでformatを使っても.parseを使わないやり方を選ばねばならない
    //.to_stringにしてそれを出力するくらいが限界。

    for i  in 0..bit.len(){
        let div = ( 1 << i);
        bit[bit.len()-1-i]  = ((decimal / div) % 2) as u8;
    }
    let zero = bit.iter().position(|&u| u == 1).unwrap() -1;
    let ret = bit[zero..].iter()
    .map(|&u| u.to_string()).collect::<String>().parse::<usize>().unwrap();

    return ret;
    
}

/*
void Binary(int x) {
    int bit[30];
    for (int i = 0; i < 30; i++) {
        int Div = (1 << i);
        bit[i] = (x / Div) % 2;
    }
}
*/