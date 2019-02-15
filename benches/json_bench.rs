// Copyright (c) 2018 R Pratap Chakravarthy.

#![feature(test)]
extern crate test;

use std::fmt::Write;
use test::Bencher;

use jsondata::Json;

#[bench]
fn bench_null(b: &mut Bencher) {
    b.iter(|| "null".parse::<Json>().unwrap());
}

#[bench]
fn bench_bool(b: &mut Bencher) {
    b.iter(|| "false".parse::<Json>().unwrap());
}

#[bench]
fn bench_num(b: &mut Bencher) {
    b.iter(|| "123121.2234234".parse::<Json>().unwrap());
}

#[bench]
fn bench_hexnum(b: &mut Bencher) {
    b.iter(|| "0x1235abcd".parse::<Json>().unwrap());
}

#[bench]
fn bench_string(b: &mut Bencher) {
    let s = r#""汉语 / 漢語; Hàn\b \tyǔ ""#;
    b.iter(|| s.parse::<Json>().unwrap());
}

#[bench]
fn bench_array(b: &mut Bencher) {
    let s = r#" [null,true,false,10,"tru\"e"]"#;
    b.iter(|| s.parse::<Json>().unwrap());
}

#[bench]
fn bench_map(b: &mut Bencher) {
    let s = r#"{"a": null,"b":true,"c":false,"d\"":-10E-1,"e":"tru\"e"}"#;
    b.iter(|| s.parse::<Json>().unwrap());
}

#[bench]
fn bench_null_to_json(b: &mut Bencher) {
    let val = "null".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_bool_to_json(b: &mut Bencher) {
    let val = "false".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_num_to_json(b: &mut Bencher) {
    let val = "10.2".parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_string_to_json(b: &mut Bencher) {
    let inp = r#""汉语 / 漢語; Hàn\b \tyǔ ""#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_array_to_json(b: &mut Bencher) {
    let inp = r#" [null,true,false,10,"tru\"e"]"#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_map_to_json(b: &mut Bencher) {
    let inp = r#"{"a": null,"b":true,"c":false,"d\"":-10E-1,"e":"tru\"e"}"#;
    let val = inp.parse::<Json>().unwrap();
    let mut outs = String::with_capacity(64);
    b.iter(|| {
        outs.clear();
        write!(outs, "{}", val)
    });
}

#[bench]
fn bench_deferred(b: &mut Bencher) {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    b.iter(|| inp.parse::<Json>().unwrap());
}

#[bench]
fn bench_no_deferred(b: &mut Bencher) {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    b.iter(|| inp.parse::<Json>().unwrap().compute().unwrap());
}

#[bench]
fn bench_json5_num(b: &mut Bencher) {
    let inp = r#" -Infinity"#;
    b.iter(|| inp.parse::<Json>().unwrap().compute().unwrap());
}
