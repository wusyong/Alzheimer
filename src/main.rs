mod inter;

use inter::IntepreterBuilder;
use std::env;
use async_std::{
    fs::File,
    io::ReadExt,
};

#[async_std::main]
async fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        std::process::exit(1);
    }
    
    let mut file = File::open(&argv[1]).await.expect("File not found.");
    let mut inputs = String::new();
    file.read_to_string(&mut inputs).await.expect("Fail to read file"); 

    IntepreterBuilder::new(inputs).await
        .build().await
        .run().await;
}
