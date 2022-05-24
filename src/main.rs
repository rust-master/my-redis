use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    client.set("hi", "redis".into()).await?;

    let result1 = client.get("hello").await?;
    let result2 = client.get("hi").await?;

    println!("got value from the server; result={:?}", result1);
    println!("got value from the server; result={:?}", result2);

    Ok(())
}
