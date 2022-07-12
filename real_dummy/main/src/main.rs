use core::blockchain::BlockChain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = BlockChain::new();

    thread::sleep(Duration::from_secs(2));
    bc.add_block("tx 1".to_string());
    println!("Add one block");
    
    thread::sleep(Duration::from_secs(2));
    bc.add_block("tx 2".to_string());
    println!("Add one block");
    
    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}\n", b);
    }
} 