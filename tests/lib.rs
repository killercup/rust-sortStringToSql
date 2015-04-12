use std::borrow::ToOwned;

extern crate sort_str_to_sql;
use sort_str_to_sql::sort_str_to_sql;

#[test]
fn it_actually_works() {
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
        let sql = match _sql { None => None, Some(s) => Some(s.to_owned())};
        assert!(
            sort_str_to_sql(input) == sql,
            "FAILED `{:?}` => `{:?}` NOT `{:?}`", input, sort_str_to_sql(input), sql
        )
    }
}
