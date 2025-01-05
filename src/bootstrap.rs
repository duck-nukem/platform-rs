use dotenv;

pub fn read_secret_from_env() -> [u8; 64] {
    dotenv::dotenv().ok();

    let mut secret: [u8; 64] = [0; 64];
    secret.copy_from_slice(
        std::env::var("APP_SECRET")
            .expect("App Secret is either undefined or not exactly 64 char long!")
            .as_bytes(),
    );

    secret
}
