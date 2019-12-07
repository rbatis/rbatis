use crate::engines::rbatis_engine::runtime::{OptMap, parser_tokens};
use test::Bencher;

#[bench]
fn bench_is_opt(b: &mut Bencher) {


    let optMap=OptMap::new();
    b.iter(|| {
       optMap.is_opt("+");
    });
}

#[bench]
fn bench_parser_tokens(b: &mut Bencher) {
    let m= &OptMap::new();
    b.iter(|| {
        parser_tokens(&String::from(" a + b"), m);
    });
}