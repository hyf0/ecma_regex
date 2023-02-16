use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecma_regex::Regex as EcmaRegex;
use regress::Regex as RegressRegex;
use rs_regex::Regex;

const TEXT: &'static str = "0123bcde89";

fn regex(c: &mut Criterion) {
    c.bench_function("regex", |b| {
        b.iter(|| Regex::new(r"^bc(d|e)*$").unwrap().is_match(black_box(TEXT)))
    });
}

fn regress(c: &mut Criterion) {
    c.bench_function("regress", |b| {
        b.iter(|| {
            RegressRegex::new(r"^bc(d|e)*$")
                .unwrap()
                .find(black_box(TEXT))
                .is_some()
        })
    });
}

fn ecma_regex(c: &mut Criterion) {
    c.bench_function("ecma_regex", |b| {
        b.iter(|| {
            EcmaRegex::new(r"^bc(d|e)*$")
                .unwrap()
                .is_match(black_box(TEXT))
        })
    });
}

fn regex_pre_compile(c: &mut Criterion) {
    c.bench_function("regex_pre_compile", |b| {
        let r = Regex::new(r"^bc(d|e)*$").unwrap();
        b.iter(|| r.is_match(black_box(TEXT)))
    });
}

fn regress_pre_compile(c: &mut Criterion) {
    c.bench_function("regress_pre_compile", |b| {
        let r = RegressRegex::new(r"^bc(d|e)*$").unwrap();
        b.iter(|| r.find(black_box(TEXT)).is_some())
    });
}

fn ecma_regex_pre_compile(c: &mut Criterion) {
    c.bench_function("ecma_regex_pre_compile", |b| {
        let r = EcmaRegex::new(r"^bc(d|e)*$").unwrap();
        b.iter(|| r.is_match(black_box(TEXT)))
    });
}

criterion_group!(
    benches,
    regex,
    ecma_regex,
    regress,
    regex_pre_compile,
    ecma_regex_pre_compile,
    regress_pre_compile,
);
criterion_main!(benches);
