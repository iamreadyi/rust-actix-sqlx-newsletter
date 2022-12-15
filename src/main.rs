use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address)?;
    //let port = listener.local_addr().unwrap().port();
    run(listener, connection)?.await
}
