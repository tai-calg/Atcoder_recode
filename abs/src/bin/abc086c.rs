use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {n :isize};
    let mut dvec:Vec<[isize;3]> = Vec::new();
    dvec.push([0,0,0]);
    for _ in 0..n {
        input! {ti:isize, xi:isize,yi:isize};
        dvec.push([ti,xi,yi]);
    }

    let mut dnext = dvec.clone();
    dnext = dnext.split_off(1);
    



    let mut flg = true;
    let len = dnext.len();

        for i in 0..len{
            let step = dnext[i][0]-dvec[i][0];
            let distance = num::abs(dvec[i][1] - dnext[i][1])+ num::abs(dvec[i][2] - dnext[i][2]) ;
            if (step < distance) | ((step - distance) % 2 != 0){
                flg = false;
                break;
            }else {
                continue;
            }
        }


    if flg{
        println!("{}", "Yes");
    }else {
        println!("{}","No");
    }
}
