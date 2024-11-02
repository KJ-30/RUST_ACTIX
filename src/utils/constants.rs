use lazy_static::lazy_static;
lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
}

fn set_address() -> String {
    let address = std::env::var("ADDRESS").expect("ADDRESS must be set");
    return address;
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    std::env::var("PORT")
        .unwrap_or("5050".to_owned())
        .parse::<u16>()
        .expect("Can't parse the port")
}
