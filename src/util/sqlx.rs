pub fn error_is_unique_violation(err: &anyhow::Error) -> bool {
    err.downcast_ref::<sqlx::error::Error>()
        .and_then(|e| e.as_database_error())
        .map(|database_error| database_error.is_unique_violation())
        .unwrap_or(false)
}

pub fn escape_like_special_chars(s: &str) -> String {
    s.replace("%", "\\%").replace("_", "\\_")
}
