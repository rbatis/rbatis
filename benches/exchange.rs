#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use test::Bencher;
use rbdc::impl_exchange;

#[bench]
fn bench_rbs_decode(b: &mut Bencher) {
    fn exchange(sql: &str) -> String {
        impl_exchange("$", 1, sql)
    }
    b.iter(|| {
        exchange("insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)");
    });
}