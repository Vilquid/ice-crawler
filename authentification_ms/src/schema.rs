diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Varchar)]
struct Username(String);

impl Username {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}