pub trait ProtoBuilder {
    fn graphql_enum(self, path: impl AsRef<str>) -> Self;

    fn graphql_input(self, path: impl AsRef<str>) -> Self;

    fn graphql_object(self, path: impl AsRef<str>) -> Self;

    fn db_enum(self, path: impl AsRef<str>, pg_type: &str) -> Self;

    fn serde(self, path: impl AsRef<str>) -> Self;
}

impl ProtoBuilder for tonic_prost_build::Builder {
    fn graphql_enum(self, path: impl AsRef<str>) -> Self {
        let attr = proc_attr(Some("graphql"), "derive(async_graphql::Enum)");
        self.type_attribute(path, attr)
    }

    fn graphql_input(self, path: impl AsRef<str>) -> Self {
        let attr = proc_attr(Some("graphql"), "derive(async_graphql::InputObject)");
        self.type_attribute(path, attr)
    }

    fn graphql_object(self, path: impl AsRef<str>) -> Self {
        let attr = proc_attr(Some("graphql"), "derive(async_graphql::SimpleObject)");
        self.type_attribute(path, attr)
    }

    fn db_enum(self, path: impl AsRef<str>, pg_type: &str) -> Self {
        let attr = proc_attr(
            Some("db"),
            &format!(
                "derive(diesel_derive_enum::DbEnum), PgType = \"{pg_type}\", DbValueStyle = \"SCREAMING_SNAKE_CASE\""
            ),
        );
        self.type_attribute(path, attr)
    }

    fn serde(self, path: impl AsRef<str>) -> Self {
        let attr = proc_attr(
            Some("serde"),
            "derive(serde::Serialize, serde::Deserialize)",
        );
        self.type_attribute(path, attr)
    }
}

fn proc_attr(feature: Option<&str>, attr: &str) -> String {
    match feature {
        Some(feat) => format!("#[cfg_attr(feature = \"{feat}\", {attr})]"),
        None => format!("#[{attr}]"),
    }
}
