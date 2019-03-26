use crate::lib::RustExpressionEngine::runtime::OptMap;
use test::Bencher;

#[bench]
fn Bench_isOpt(b: &mut Bencher) {


    let optMap=OptMap::new();
    b.iter(|| {
       optMap.isOpt("+");
    });
}