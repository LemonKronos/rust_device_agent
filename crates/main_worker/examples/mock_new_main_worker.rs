
// use std::fs;
// use main_worker::info_gatherer;
// use shared_libs::utils::*;

#[tokio::main]
async fn main() {    
    println!("If this show up, the binary have been udpated");

    loop {
        println!("New binary running");
        let sleep_time = tokio::time::Instant::now() + std::time::Duration::from_secs(2);
        tokio::time::sleep_until(sleep_time).await;
    }
}