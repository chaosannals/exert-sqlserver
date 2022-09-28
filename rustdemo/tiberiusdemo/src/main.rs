use tiberius::{Client, Config, AuthMethod, error::Error, EncryptionLevel, Query};
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    config.host("127.0.0.1");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "sa.pass00.word11"));
    config.encryption(EncryptionLevel::Off);
    config.trust_cert(); // 不提倡生产环境使用，但是不加这句链接会失败。

    println!("aaaaa2");

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    println!("aaaaa1");

    let mut client = match Client::connect(config, tcp.compat_write()).await {
        // Connection successful.
        Ok(client) => client,
        // The server wants us to redirect to a different address
        Err(Error::Routing { host, port }) => {
            println!("aaaaa4");
            let mut config = Config::new();

            config.host(&host);
            config.port(port);
            config.authentication(AuthMethod::sql_server("tester", "123456"));

            println!("aaaaa3");
            let tcp = TcpStream::connect(config.get_addr()).await?;
            tcp.set_nodelay(true)?;

            // we should not have more than one redirect, so we'll short-circuit here.
            Client::connect(config, tcp.compat_write()).await?
        }
        Err(e) => Err(e)?,
    };
    println!("aaaaa10");
    let mut select = Query::new("SELECT TOP 10 Name from sys.databases WHERE Name LIKE @P1;");
    select.bind("%m%");
    let stream = select.query(&mut client).await?;
    let row = stream.into_row().await?;

    println!("{:?}\n", row);

    let params = vec![String::from("foo"), String::from("bar"), String::from("333")];
    let mut select = Query::new("SELECT @P1, @P2, @P3");

    for param in params.into_iter() {
        select.bind(param);
    }

    let res = select.query(&mut client).await?;

    println!("{:?}\n", res);

    Ok(())
}