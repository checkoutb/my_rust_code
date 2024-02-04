
use mini_redis::{client, Result};


// /target 增加 400mb。。。？？？

#[tokio::main]
async fn main() -> Result<()> {

    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("server's response: {:?}", result);
    
    Ok(())
}