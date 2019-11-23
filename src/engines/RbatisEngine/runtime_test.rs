use crate::engines::RbatisEngine::runtime::{OptMap, ParserTokens};
use test::Bencher;

#[bench]
fn Bench_isOpt(b: &mut Bencher) {


    let optMap=OptMap::new();
    b.iter(|| {
       optMap.isOpt("+");
    });
}

#[bench]
fn BenchParserTokens(b: &mut Bencher) {
    let m= &OptMap::new();
    b.iter(|| {
        ParserTokens(&String::from(" a + b"), m);
    });
}