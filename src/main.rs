mod bot;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    bot::start_bot().await 
}
