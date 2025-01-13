use std::collections::HashMap;

pub trait SerializableAsUrl {
    fn as_url(&self) -> &'static str;
}

pub enum Prefix {
    Root,
    Nested(&'static str),
}

pub enum QueryParams {
    None,
    From(Vec<(String, String)>),
}

#[allow(clippy::needless_pass_by_value)]
pub fn build_url(
    prefix: Prefix,
    route_name: impl SerializableAsUrl + 'static,
    query_params: QueryParams,
) -> String {
    let path = match prefix {
        Prefix::Root => route_name.as_url().to_string(),
        Prefix::Nested(parent) => format!(
            "/{}/{}",
            parent.replace('/', ""),
            route_name.as_url().replace('/', "")
        ),
    };
    let params: HashMap<String, String> = match query_params {
        QueryParams::From(items) => items.into_iter().collect(),
        QueryParams::None => return path,
    };

    let mut params_stringified = params
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<String>>();
    params_stringified.sort();

    format!("{}?{}", path, params_stringified.join("&"))
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestRoute {
        Login,
        Slashed,
    }

    impl SerializableAsUrl for TestRoute {
        fn as_url(&self) -> &'static str {
            match self {
                TestRoute::Login => "/login",
                TestRoute::Slashed => "/slash/",
            }
        }
    }

    #[tokio::test]
    async fn test_build_url_with_prefix_and_without_query_params() {
        let result = build_url(Prefix::Nested("auth"), TestRoute::Login, QueryParams::None);

        assert_eq!("/auth/login", result);
    }

    #[tokio::test]
    async fn test_build_url_with_prefix_and_query_params() {
        let result = build_url(
            Prefix::Root,
            TestRoute::Login,
            QueryParams::From(vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string()),
            ]),
        );

        assert_eq!("/login?a=1&b=2", result);
    }

    #[tokio::test]
    async fn test_build_url_without_prefix_and_without_query_params() {
        let result = build_url(Prefix::Root, TestRoute::Login, QueryParams::None);

        assert_eq!("/login", result);
    }

    #[tokio::test]
    async fn test_should_deduplicate_forward_slashes() {
        let result = build_url(
            Prefix::Nested("//auth/"),
            TestRoute::Slashed,
            QueryParams::None,
        );

        assert_eq!("/auth/slash", result);
    }
}
