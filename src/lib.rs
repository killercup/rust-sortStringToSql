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
//! # use sort_str_to_sql::{sort_str_to_sql};
//! assert!(
//!   sort_str_to_sql("-id") ==
//!   Some("id DESC NULLS LAST".to_string())
//! )
//! assert!(
//!   sort_str_to_sql("-id,+aired-") ==
//!   Some("id DESC NULLS LAST, aired ASC NULLS FIRST".to_string())
//! )
//! ```
//!
//! (See tests for more examples.)
#![crate_name = "sort_str_to_sql"]

#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

static SORT_STR_FORMAT: regex::Regex = regex!(
    r"^(?P<order>[+-]?)(?P<field>[\w.]+)(?P<nulls>[-]?)$"
);

/// Convert One Sort Expression to SQL
fn convert_one_sort_str_field_to_sql(sort_str: &str) -> Option<String> {
    let fields = match SORT_STR_FORMAT.captures(sort_str) {
        Some(captures) => captures,
        None => return None,
    };

    let field = fields.name("field");
    let order = match fields.name("order") {
        "-" => "DESC",
        _   => "ASC",
    };
    let nulls = match fields.name("nulls") {
        "-" => "NULLS FIRST",
        _   => "NULLS LAST",
    };

    let sql = format!("{} {} {}", field, order, nulls);
    return Some(sql);
}

/// Convert Sort Expression to SQL
pub fn sort_str_to_sql(sort_str: &str) -> Option<String> {
    let split_mark = regex!("[,]");

    let sql_array: Vec<String> = split_mark.split(sort_str)
    .map(convert_one_sort_str_field_to_sql)
    .filter(|input| input.is_some())
    .map(|input| input.unwrap())
    .collect();

    let sql = sql_array.connect(", ").to_string();
    return if sql == "".to_string() { None } else { Some(sql) };
}

#[test]
fn it_works() {
    let tests = vec![
        // Correct inputs
        ("id", Some("id ASC NULLS LAST")),
        ("+id", Some("id ASC NULLS LAST")),
        ("-id", Some("id DESC NULLS LAST")),
        ("id-", Some("id ASC NULLS FIRST")),
        ("+id-", Some("id ASC NULLS FIRST")),
        ("-id-", Some("id DESC NULLS FIRST")),
        ("show.id", Some("show.id ASC NULLS LAST")),
        ("-id,aired-", Some("id DESC NULLS LAST, aired ASC NULLS FIRST")),
        ("-id,+aired-", Some("id DESC NULLS LAST, aired ASC NULLS FIRST")),
        ("+id-,show.id", Some("id ASC NULLS FIRST, show.id ASC NULLS LAST")),

        // Incorrect inputs
        ("lol what", None),
        ("+-id-", None),

        // Partially correct inputs
        ("id,++aired+", Some("id ASC NULLS LAST")),
        ("?id,+aired-", Some("aired ASC NULLS FIRST")),
    ];

    for &(input, _sql) in tests.iter() {
        let sql = match _sql { None => None, Some(str) => Some(str.to_string())};
        assert!(
            sort_str_to_sql(input) == sql,
            "FAILED `{}` => `{}` NOT `{}`", input, sort_str_to_sql(input), sql
        )
    }
}
