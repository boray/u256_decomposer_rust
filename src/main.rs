use std::io::{stdin,stdout,Write};
use primitive_types::U256;

fn main() {
    let mut s=String::new();
    print!("Enter U256 Integer to decompose into [u64;4]: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    let U256(ref words) = U256::from_dec_str(&s).unwrap();
    
    println!("a: {},", words[0]);
    println!("b: {},", words[1]);
    println!("c: {},", words[2]);
    println!("d: {}", words[3]);
}
 
//fn lo(&self) -> u64 { *self as u64 }
//fn hi(&self) -> u64 { (*self >> 64) as u64 }