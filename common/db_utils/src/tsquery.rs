use regex::Regex;
use std::sync::LazyLock;

static SPLIT_WORDS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\w+").unwrap());
const PG_TSQUERY_OPERATOR: PgTsqueryOperator = PgTsqueryOperator {
    and: "&",
    or: "|",
    not: "!",
    followed_by: "<->",
};

#[allow(unused)]
struct PgTsqueryOperator {
    and: &'static str,
    or: &'static str,
    not: &'static str,
    followed_by: &'static str,
}

pub fn maybe_ts_query_raw(query_str: Option<&str>) -> Option<String> {
    let query_str = query_str?;
    let splitted = SPLIT_WORDS_REGEX
        .find_iter(query_str)
        .map(|mat| mat.as_str().to_string())
        .collect::<Vec<_>>();
    (!splitted.is_empty()).then_some(splitted.join(&format!(" {} ", PG_TSQUERY_OPERATOR.or)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_words_regex_not_panic() {
        maybe_ts_query_raw(Some("The quick brown fox jumps over the lazy dog"));
    }
}
