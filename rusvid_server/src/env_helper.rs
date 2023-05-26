use std::fmt::Debug;
use std::str::FromStr;

use tracing::debug;

#[inline(always)]
#[doc(hidden)]
fn get_env_var<T: FromStr + Clone + Debug>(key: &str, default: T) -> T {
    let value = std::env::var(key)
        .map(|raw| raw.parse::<T>().unwrap_or(default.clone()))
        .unwrap_or(default);

    debug!("env: {{ key {:?}, value {:?} }}", key, value);

    value
}

/// Helper macro to generate env helper functions.
/// ```no_run
/// generate_helper!(api_port, api_port, 8080, u16);
///
/// // Expands to
/// fn api_port() -> u16 {
///     get_env_var("api_port", 8080)
/// }
/// ```
macro_rules! generate_helper {
    ($fct_name:ident, $env_name:tt, $default:expr, $type:ty) => {
        #[doc = "helper method to get env variable with key `"]
        #[doc = stringify!($env_name)]
        #[doc = "`."]
        #[doc = ""]
        #[doc = "Default value: `"]
        #[doc = stringify!($default)]
        #[doc = "`"]
        pub fn $fct_name() -> $type {
            get_env_var(stringify!($env_name), $default)
        }
    };
}

generate_helper!(api_port, API_PORT, 8080, u16);
generate_helper!(metrics_port, METRICS_PORT, 8085, u16);
generate_helper!(
    s3_access_key,
    STORAGE_ACCESS_KEY,
    "access_key_123".to_string(),
    String
);
generate_helper!(
    s3_secret_key,
    STORAGE_SECRET_KEY,
    "access_secret_key_123".to_string(),
    String
);
generate_helper!(
    s3_bucket,
    STORAGE_BUCKET,
    "rusvid-media".to_string(),
    String
);
generate_helper!(s3_url, STORAGE_URL, "127.0.0.1:9000".to_string(), String);
generate_helper!(s3_region, STORAGE_REGION, "home".to_string(), String);
generate_helper!(redis_url, REDIS_URL, "127.0.0.1".to_string(), String);

generate_helper!(
    exporter_endpoint,
    EXPORTER_URL,
    "http://127.0.0.1:4317".to_string(),
    String
);
