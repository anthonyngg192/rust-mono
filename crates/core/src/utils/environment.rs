use std::env;

use once_cell::sync::Lazy;

pub static DB_CONNECTION_STRING: Lazy<String> =
    Lazy::new(|| env::var("DB_CONNECTION_STRING").expect("Missing DB_CONNECTION_STRING variable"));

pub static DATABASE_NAME: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_NAME").expect("Missing DATABASE_NAME variable"));

pub static API_PORT: Lazy<usize> = Lazy::new(|| {
    env::var("API_PORT")
        .expect("Missing API_PORT variable")
        .parse()
        .unwrap()
});

pub static HASH_ROUND: Lazy<u32> = Lazy::new(|| {
    env::var("HASH_ROUND")
        .expect("Missing HASH_ROUND variable")
        .parse()
        .unwrap()
});

pub static APP_NAME: Lazy<String> =
    Lazy::new(|| env::var("APP_NAME").expect("Missing APP_NAME variable"));

pub static JWT_SECRET_KEY: Lazy<String> =
    Lazy::new(|| env::var("JWT_SECRET_KEY").expect("Missing JWT_SECRET_KEY variable"));

pub static LIVEKIT_API_KEY: Lazy<String> =
    Lazy::new(|| env::var("LIVEKIT_API_KEY").expect("Missing LIVEKIT_API_KEY variable"));

pub static LIVEKIT_API_SECRET: Lazy<String> =
    Lazy::new(|| env::var("LIVEKIT_API_SECRET").expect("Missing LIVEKIT_API_SECRET variable"));

pub static LIVEKIT_HOST: Lazy<String> =
    Lazy::new(|| env::var("LIVEKIT_HOST").expect("Missing LIVEKIT_HOST variable"));

pub static WEBHOOK_API_SIGNED_KEY: Lazy<String> = Lazy::new(|| {
    env::var("WEBHOOK_API_SIGNED_KEY").expect("Missing WEBHOOK_API_SIGNED_KEY variable")
});

pub static SALT_STRING: Lazy<String> =
    Lazy::new(|| env::var("SALT_STRING").expect("Missing SALT_STRING variable"));
