//! # Convert Sort Expression to SQL
//!
//! Converts 'sort format' to 'SQL format'. E.g.:
//!
//! - `-id` -> `id DESC NULLS LAST`
//! - `id`  -> `id ASC NULLS LAST`
//! - `id-` -> `id ASC NULLS FIRST`
//! - `-aired,id` -> `aired DESC NULLS LAST, id ASC NULLS LAST`
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
pub fn sort_str_to_sql(sort_str: &str) -> String {
  let split_mark = regex!("[,]");

  let sql_array: Vec<String> = split_mark.split(sort_str)
  .map(convert_one_sort_str_field_to_sql)
  .filter(|input| input.is_some())
  .map(|input| input.unwrap())
  .collect();

  sql_array
  .connect(", ")
  .to_string()
}

#[test]
fn it_works() {
  let tests = vec![
    // Correct inputs
    ("id", "id ASC NULLS LAST"),
    ("+id", "id ASC NULLS LAST"),
    ("-id", "id DESC NULLS LAST"),
    ("id-", "id ASC NULLS FIRST"),
    ("+id-", "id ASC NULLS FIRST"),
    ("-id-", "id DESC NULLS FIRST"),
    ("show.id", "show.id ASC NULLS LAST"),
    ("-id,aired-", "id DESC NULLS LAST, aired ASC NULLS FIRST"),
    ("-id,+aired-", "id DESC NULLS LAST, aired ASC NULLS FIRST"),
    ("+id-,show.id", "id ASC NULLS FIRST, show.id ASC NULLS LAST"),

    // Incorrect inputs
    ("lol what", ""),
    ("+-id-", ""),

    // Partially correct inputs
    ("id,++aired+", "id ASC NULLS LAST"),
    ("?id,+aired-", "aired ASC NULLS FIRST"),
  ];

  for &(input, sql) in tests.iter() {
    assert!(
      sort_str_to_sql(input) == sql.to_string(),
      "FAILED `{}` => `{}` NOT `{}`", input, sort_str_to_sql(input), sql
    )
  }
}
