#![feature(test)]
extern crate test;

use rbatis::wrapper::Wrapper;
use rbatis_core::db::DriverType;
use test::Bencher;

#[bench]
fn bench_wrapper(b: &mut Bencher) {
    b.iter(|| {
        Wrapper::new(&DriverType::Postgres)
            .having("id").and()
            .eq("id", 1).and()
            .ne("id", 1).and()
            .gt("id", 1).and()
            .ge("id", 1).and()
            .lt("id", 1).and()
            .le("id", 1).and()
            .between("id", 1, 2).and()
            .not_between("id", 1, 2).and()
            .like("id", 1).and()
            .like_left("id", 1).and()
            .like_right("id", 1).and()
            .not_like("id", 1).and()
            .is_null("id").and()
            .is_not_null("id").and()
            .in_array("id", &[1]).and()
            .not_in("id", &[1]).and()
            .order_by(true, &["id"])
            .group_by(&["id"])
            .limit(1)
            .order_bys(&[("id", true), ("name", false)]);
    });
}

#[bench]
fn bench_ser_json(b: &mut Bencher) {
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct A {
        pub name: String,
    }
    let mut buf = String::with_capacity(1000000);
    for _ in 0..1000000 {
        buf.push_str("a");
    }
    let a = A {
        name: buf
    };
    b.iter(|| {
        let buf = serde_json::to_vec(&a).unwrap();
    });
}

#[bench]
fn bench_ser(b: &mut Bencher) {
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct A {
        pub name: String,
    }
    let mut buf = String::with_capacity(1000000);
    for _ in 0..1000000 {
        buf.push_str("a");
    }
    let a = A {
        name: buf
    };
    b.iter(|| {
        let buf = rbmp::to_vec(&a).unwrap();
    });
}

#[bench]
fn bench_ser_zero(b: &mut Bencher) {
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct A {
        pub name: String,
    }
    let mut buf = String::with_capacity(1000000);
    for _ in 0..1000000 {
        buf.push_str("a");
    }
    let a = A {
        name: buf
    };
    b.iter(|| {
        let buf = rbmp::to_value_ref(&a).unwrap();
    });
}