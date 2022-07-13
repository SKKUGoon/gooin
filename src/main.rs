use blockchainlib::*;

fn main() {
    let block = Block::new(130, now(), vec![0; 32], 0, "genesis block".to_owned());
    println!("{:?}", &block);
}
