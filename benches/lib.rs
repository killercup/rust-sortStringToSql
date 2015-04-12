#![feature(test)]

extern crate test;
use test::Bencher;

extern crate sort_str_to_sql;
use sort_str_to_sql::sort_str_to_sql;

#[bench]
fn bench_simple(b: &mut Bencher) {
    b.iter(|| sort_str_to_sql("id"));
}

#[bench]
fn bench_complex(b: &mut Bencher) {
    b.iter(|| sort_str_to_sql("-date,+id-,show.id"));
}