use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = match client::connect("localhost:6379").await {
        Ok(client) => client,
        Err(_) => panic!("failed to establish connection"),
    };

    // difference between ? and unwrap is that
    // ? will return an error
    // however, unwrap will panic

    // into is used for converting a string to a byte array
    client.set("foo", "bar".into()).await?;

    let result = client.get("foo").await?.unwrap();

    println!(
        "the result is: {:?}",
        String::from_utf8(result.to_vec()).unwrap()
    );

    Ok(())
}
