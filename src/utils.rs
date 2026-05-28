// !src/utils.rs
use url::Url;
use rand::Rng;

const BASE62: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
pub fn generate_short_code() -> String {
    let mut rng = rand::thread_rng();
    (0..7).map(|_| {
        let idx = rng.gen_range(0..BASE62.len());
        BASE62[idx] as char
    })
    .collect()
}

pub fn validate_url(input: &str) -> bool {
    Url::parse(input).is_ok()
}
