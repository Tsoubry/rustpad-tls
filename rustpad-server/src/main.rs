use rustpad_server::{database::Database, server, ServerConfig};
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let certificate_path =
        std::env::var("TLS_CERT_PATH").expect("Wasn't able to parse TLS_CERT_PATH");
    let key_path = std::env::var("TLS_KEY_PATH").expect("Wasn't able to parse TLS_KEY_PATH");

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| String::from("3030"))
        .parse()
        .expect("Unable to parse PORT");

    let addr = IpAddr::from([0; 8]);

    let config = ServerConfig {
        expiry_days: std::env::var("EXPIRY_DAYS")
            .unwrap_or_else(|_| String::from("1"))
            .parse()
            .expect("Unable to parse EXPIRY_DAYS"),
        database: match std::env::var("SQLITE_URI") {
            Ok(uri) => Some(
                Database::new(&uri)
                    .await
                    .expect("Unable to connect to SQLITE_URI"),
            ),
            Err(_) => None,
        },
    };

    warp::serve(server(config))
        .tls()
        .cert_path(&certificate_path)
        .key_path(&key_path)
        .run((addr, port))
        .await;
}
