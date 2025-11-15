use crate::api::Api;

pub type Ret<T> = Result<T, Box<dyn std::error::Error>>;

pub fn best_key(keys: &[String]) -> Ret<String> {
    let mut best_key = None;
    let mut max_searches = 0;

    for key in keys {
        if let Ok(remaining) = Api::new(key).account_info() {
            if remaining > max_searches {
                max_searches = remaining;
                best_key = Some(key.clone());
            }
        }
    }

    best_key.ok_or("No valid API keys with searches remaining".into())
}
