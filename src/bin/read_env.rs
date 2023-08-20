use std::env;

fn main() {
    dotenv::dotenv().ok(); // Load the .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);
}
