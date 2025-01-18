use std::env;
use std::str::FromStr;

pub fn read_bool_env_var(key: &str, default: bool) -> bool {
    env::var(key)
        .unwrap_or_else(|_| -> String { default.to_string().to_ascii_lowercase() })
        .eq("true")
}

pub fn read_numeric_env_var<T: FromStr + ToString>(key: &str, default: &T) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    env::var(key)
        .unwrap_or_else(|_| -> String { default.to_string() })
        .parse::<T>()
        .unwrap_or_else(|_| panic!("{}", "Non-numeric value provided for {key}"))
}

pub fn read_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| -> String { default.to_string() })
}

pub fn read_mandatory_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        panic!(
            "{}",
            "{key} is a required environment variable, but it wasn't found!"
        )
    })
}
