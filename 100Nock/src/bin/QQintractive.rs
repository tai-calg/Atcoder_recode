
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
