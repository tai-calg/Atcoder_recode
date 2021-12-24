use proconio::{input, fastout};


#[fastout]
fn main() {
    input! {N : u8};
    input!{mut A : [u8;N]};

    let mut bob_sum :u128= 0;
    let mut alice_sum: u128 = 0;

    let mut is_alice = true;

    //先にソートしてreverseしてpopすればいいだけだった。。。
    
    while !(A.is_empty()) {
        
        let max = A.iter().max().unwrap();
        let max_index = Index_max(&A).unwrap();
        
        if is_alice {
            alice_sum += *max as u128;
            A.remove(max_index);
            is_alice = !is_alice;
        }
        else {
            bob_sum += *max as u128;
            A.remove(max_index);
            is_alice = !is_alice;
        }
        
        
    }

    let ans = alice_sum - bob_sum;
    println!("{}", ans);



}

//Vecの最大値のインデックスを取得する関数
fn Index_max<T: Ord>(slice: &[T]) -> Option<usize> {
    slice.iter().enumerate().max_by(|(_, value0), (_, value1)| value0.cmp(value1)).map(|(idx, _)| idx)
}