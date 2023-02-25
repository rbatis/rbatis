#![feature(test)]
extern crate test;

use rbdc::impl_exchange;
use test::Bencher;

#[bench]
fn bench_rbs_decode(b: &mut Bencher) {
    fn exchange(sql: &str) -> String {
        impl_exchange("$", 1, sql)
    }
    b.iter(|| {
        exchange("insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)");
    });
}
