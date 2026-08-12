


mod ipc;

#[tokio::main]
async fn main() { 
    ipc::start_ipc_server().await; 
}