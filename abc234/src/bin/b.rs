use proconio::input;
fn main() {
    input! {n:usize};
    input! {xy: [(isize,isize);n]};

    let mut max : f64 = 0.0;

    for &xy1 in &xy{
        for &xy2 in &xy{
        let dis = distance(&xy1, &xy2) ;  
            if dis > max {
                max = dis;
            }
        }
    }

    println!("{}", max);

}

fn distance (xy1:&(isize,isize), xy2:&(isize,isize) )->f64{
    let xdif = isize::abs(xy1.0 - xy2.0) as f64 ;
    let ydif = isize::abs(xy1.1 - xy2.1) as f64;

    return f64::sqrt(xdif * xdif + ydif * ydif);
}