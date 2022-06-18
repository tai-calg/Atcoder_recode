// Your code here!

fn main(){
    let n = 100;
            for i in 0..=n {
    
            for j in (2*i..=n).step_by(i) {
    println!("{}", j);
            }
        }
    }