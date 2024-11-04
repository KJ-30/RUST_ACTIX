use lazy_static::lazy_static;
use std::env;
lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap_or("localhost".to_owned())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    std::env::var("PORT")
        .unwrap_or("5050".to_owned())
        .parse::<u16>()
        .expect("Can't parse the port")
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").expect("postgres://postgres:665712@localhost:5432/NewBlogDB")
}
