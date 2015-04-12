//! # Convert Sort Expression to SQL
//!
//! Converts 'sort format' to 'SQL format'. E.g.:
//!
//! - `-id` -> `id DESC NULLS LAST`
//! - `id`  -> `id ASC NULLS LAST`
//! - `id-` -> `id ASC NULLS FIRST`
//! - `-aired,id` -> `aired DESC NULLS LAST, id ASC NULLS LAST`
//!
//! # Example
//!
//! ```
//! #![allow(unstable)]
//! # use sort_str_to_sql::{sort_str_to_sql};
//! # use std::borrow::ToOwned;
//! assert!(
//!   sort_str_to_sql("-id") ==
//!   Some("id DESC NULLS LAST".to_owned())
//! );
//! assert!(
//!   sort_str_to_sql("-id,+aired-") ==
//!   Some("id DESC NULLS LAST, aired ASC NULLS FIRST".to_owned())
//! );
//! ```
//!
//! (See tests for more examples.)

extern crate regex;

// For rust beta
macro_rules! regex(
    ($s:expr) => (regex::Regex::new($s).unwrap());
);

/// Convert One Sort Expression to SQL
fn convert_one_sort_str_field_to_sql(sort_str: &str) -> Option<String> {
    let sort_str_format = regex!(
        r"^(?P<order>[+-]?)(?P<field>[a-zA-Z0-9_\.]+)(?P<nulls>[-]?)$"
    );

    let fields = match sort_str_format.captures(sort_str) {
        Some(captures) => captures,
        None => return None,
    };

    let field = match fields.name("field") {
        Some(f) => f,
        None => return None,
    };

    let order = match fields.name("order") {
        Some("-") => "DESC",
        _   => "ASC",
    };
    let nulls = match fields.name("nulls") {
        Some("-") => "NULLS FIRST",
        _   => "NULLS LAST",
    };

    let sql = format!("{} {} {}", field, order, nulls);
    return Some(sql);
}

/// Convert Sort Expression to SQL
pub fn sort_str_to_sql(sort_str: &str) -> Option<String> {
    let sql_array: Vec<String> = sort_str.split(',')
    .filter_map(convert_one_sort_str_field_to_sql)
    .collect();

    let sql = sql_array.connect(", ");
    return if sql == "" { None } else { Some(sql) };
}
