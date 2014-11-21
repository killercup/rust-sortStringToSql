# Sort String To SQL

This is a rust crate to convert 'sort expressions' to SQL expressions that can be used in an 'ORDER BY'. E.g., `-date,id` will be converted to `date DESC NULLS LAST, id ASC NULLS LAST`.

It can be used to convert a HTTP query parameter to an expression that can be used in an SQL query. (I'm only using this with Postgres myself, so I can't promise you it will work with other databases.)

[![Build Status](https://travis-ci.org/killercup/rust-sortStringToSql.svg)](https://travis-ci.org/killercup/rust-sortStringToSql)

## Install

```toml
# Cargo.toml
[dependencies]
sort_str_to_sql = "~0.0.1"
```

## Input Format

Comma separated list of field names. Prepend a '-' for descending order or a '+' dor ascending order (which is the default, so it's optional). Append a '-' if you want records with `null` values first (this sets `NULLS FIRST`).

You can find some examples in `test.js`.

## License

MIT
