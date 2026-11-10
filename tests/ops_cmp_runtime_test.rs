//! Runtime coverage for `rbatis-codegen`'s operator modules.
//!
//! Targets:
//!   * ops.rs (AsProxy impls + PartialEq/PartialOrd default bodies)
//!   * ops_eq.rs / ops_cmp.rs (comparison impl matrices and helper call sites)
//!   * ops_compare.rs (private helpers, reached through public operators)
//!   * from_bool.rs
//!   * into_sql.rs
//!   * error.rs
#[cfg(test)]
mod test {
    use rbatis_codegen::error::Error as CodegenError;
    use rbatis_codegen::into_sql::IntoSql;
    use rbatis_codegen::ops::{AsProxy, From as OpFrom, PartialEq as OpEq, PartialOrd as OpOrd};
    use rbs::value::map::ValueMap;
    use rbs::Value;
    use std::borrow::Cow;
    use std::cmp::Ordering;
    use std::error::Error as StdError;

    // ------------------------------------------------------------------
    // helpers
    // ------------------------------------------------------------------

    fn one_entry_map(k: Value, v: Value) -> Value {
        let mut m = ValueMap::new();
        m.insert(k, v);
        Value::Map(m)
    }

    fn ext(inner: Value) -> Value {
        Value::Ext("custom", Box::new(inner))
    }

    // Fully exercises the four owned/reference impls generated for one
    // `impl_compare_matrix!` cell: `impl PartialEq<$lhs> for $rhs` and its
    // `&`-variants. `$r` is the `self` (rhs-list) type, `$l` the argument type.
    macro_rules! eq_pair {
        ($r:expr, $l:expr) => {{
            let a = $r;
            let b = $l;
            let ra = &a;
            let rb = &b;
            assert!(a.op_eq(&b));
            assert!(a.op_eq(&rb));
            assert!(ra.op_eq(&b));
            assert!(ra.op_eq(&rb));
        }};
    }

    // One row of an `impl_compare_matrix!` invocation (self + all key types).
    macro_rules! eq_row {
        ($r:expr ; [$($l:expr),* $(,)?]) => {
            $(
                eq_pair!($r, $l);
            )*
        };
    }

    // Cartesian product over the rows of an `impl_compare_matrix!` invocation.
    macro_rules! eq_matrix {
        ([$($r:expr),* $(,)?] ; $keys:tt) => {
            $(
                eq_row!($r ; $keys);
            )*
        };
    }

    macro_rules! cmp_pair {
        ($r:expr, $l:expr) => {{
            let a = $r;
            let b = $l;
            let ra = &a;
            let rb = &b;
            assert_eq!(a.op_partial_cmp(&b), Some(Ordering::Equal));
            assert_eq!(a.op_partial_cmp(&rb), Some(Ordering::Equal));
            assert_eq!(ra.op_partial_cmp(&b), Some(Ordering::Equal));
            assert_eq!(ra.op_partial_cmp(&rb), Some(Ordering::Equal));
        }};
    }

    // One row of an `impl_compare_matrix!` invocation (self + all key types).
    macro_rules! cmp_row {
        ($r:expr ; [$($l:expr),* $(,)?]) => {
            $(
                cmp_pair!($r, $l);
            )*
        };
    }

    macro_rules! cmp_matrix {
        ([$($r:expr),* $(,)?] ; $keys:tt) => {
            $(
                cmp_row!($r ; $keys);
            )*
        };
    }

    // Exercises all five `impl_str_eq!` impls generated for one scalar type.
    macro_rules! str_eq_all {
        ($ty:ty, $lit:expr, $v:expr) => {{
            assert!(<&str as OpEq<$ty>>::op_eq(&$lit, &$v));
            assert!(<$ty as OpEq<&str>>::op_eq(&$v, &$lit));
            assert!(<$ty as OpEq<&&str>>::op_eq(&$v, &&$lit));
            assert!(<$ty as OpEq<&&&str>>::op_eq(&$v, &&&$lit));
            assert!(<&&str as OpEq<$ty>>::op_eq(&&$lit, &$v));
        }};
    }

    // Exercises all twelve `impl_value_scalar_compare_op!` impls for one scalar.
    macro_rules! eq_scalar_all {
        ($v:expr, $s:expr) => {{
            let v: Value = $v;
            let rv: &Value = &v;
            let rrv: &&Value = &rv;
            let s = $s;
            let rs = &s;
            let rrs = &rs;
            assert!(v.op_eq(&s));
            assert!(v.op_eq(&rs));
            assert!(rv.op_eq(&s));
            assert!(rv.op_eq(&rs));
            assert!(rrv.op_eq(&s));
            assert!(rrv.op_eq(&rs));
            assert!(s.op_eq(&v));
            assert!(s.op_eq(&rv));
            assert!(s.op_eq(&rrv));
            assert!(rs.op_eq(&v));
            assert!(rs.op_eq(&rv));
            assert!(rs.op_eq(&rrv));
            // keep `rrs` used so the reference chain is built in every branch
            let _ = rrs;
        }};
    }

    macro_rules! cmp_scalar_all {
        ($v:expr, $s:expr) => {{
            let v: Value = $v;
            let rv: &Value = &v;
            let rrv: &&Value = &rv;
            let s = $s;
            let rs = &s;
            assert_eq!(v.op_partial_cmp(&s), Some(Ordering::Equal));
            assert_eq!(v.op_partial_cmp(&rs), Some(Ordering::Equal));
            assert_eq!(rv.op_partial_cmp(&s), Some(Ordering::Equal));
            assert_eq!(rv.op_partial_cmp(&rs), Some(Ordering::Equal));
            assert_eq!(rrv.op_partial_cmp(&s), Some(Ordering::Equal));
            assert_eq!(rrv.op_partial_cmp(&rs), Some(Ordering::Equal));
            assert_eq!(s.op_partial_cmp(&v), Some(Ordering::Equal));
            assert_eq!(s.op_partial_cmp(&rv), Some(Ordering::Equal));
            assert_eq!(s.op_partial_cmp(&rrv), Some(Ordering::Equal));
            assert_eq!(rs.op_partial_cmp(&v), Some(Ordering::Equal));
            assert_eq!(rs.op_partial_cmp(&rv), Some(Ordering::Equal));
            assert_eq!(rs.op_partial_cmp(&rrv), Some(Ordering::Equal));
        }};
    }

    // ------------------------------------------------------------------
    // ops.rs: AsProxy for Value
    // ------------------------------------------------------------------

    #[test]
    fn test_asproxy_value_numeric() {
        assert_eq!(Value::I32(7).i32(), 7);
        assert_eq!(Value::I64(-7).i64(), -7);
        assert_eq!(Value::U32(7).u32(), 7);
        assert_eq!(Value::U64(7).u64(), 7);
        assert_eq!(Value::U64(7).usize(), 7);
        assert_eq!(Value::F32(2.5).f64(), 2.5);
        assert_eq!(Value::F64(2.5).f64(), 2.5);
        assert_eq!(Value::I64(-2).i32(), -2);
        assert_eq!(Value::U32(9).i64(), 9);

        // out-of-range / negative casts hit the `as` truncation paths
        assert_eq!(Value::I64(i64::MAX).i32(), -1);
        assert_eq!(Value::U64(u64::MAX).u32(), u32::MAX);
        assert_eq!(Value::U64(u64::MAX).usize(), usize::MAX);
        assert_eq!(Value::F64(-1.5).u64(), 0);
        assert_eq!(Value::F64(-1.5).i64(), -1);

        // non numeric -> `unwrap_or_default()`
        assert_eq!(Value::Null.i32(), 0);
        assert_eq!(Value::Bool(true).i64(), 0);
        assert_eq!(Value::String("12".into()).i32(), 0);
        assert_eq!(Value::Array(vec![]).u32(), 0);
        assert_eq!(Value::Binary(vec![1]).u64(), 0);
        assert_eq!(Value::Map(ValueMap::new()).f64(), 0.0);
        assert_eq!(Value::Map(ValueMap::new()).usize(), 0);
        assert_eq!(Value::Ext("x", Box::new(Value::Null)).i64(), 0);

        // Ext delegates to the wrapped value
        assert_eq!(ext(Value::I64(9)).i64(), 9);
        assert_eq!(ext(Value::U32(9)).u32(), 9);
        assert_eq!(ext(Value::U64(9)).u64(), 9);
        assert_eq!(ext(Value::F64(9.5)).f64(), 9.5);
        assert_eq!(ext(Value::I32(9)).i32(), 9);
        assert_eq!(ext(Value::I32(9)).usize(), 9);
    }

    #[test]
    fn test_asproxy_value_string_bool_binary() {
        assert_eq!(Value::String("abc".into()).string(), "abc");
        assert_eq!(ext(Value::String("ext".into())).string(), "ext");
        // Ext over a non-string: `as_string()` is None -> default empty string
        assert_eq!(ext(Value::I32(5)).string(), "");
        assert_eq!(Value::I32(5).string(), "5");
        assert_eq!(Value::U64(5).string(), "5");
        assert_eq!(Value::F64(1.5).string(), "1.5");
        assert_eq!(Value::Bool(true).string(), "true");
        assert_eq!(Value::Null.string(), "null");
        assert_eq!(Value::Array(vec![Value::I32(1)]).string(), "[1]");
        assert_eq!(Value::Binary(vec![1, 2]).string(), "[1, 2]");
        assert!(Value::Map(ValueMap::new()).string().contains('{'));

        assert!(Value::Bool(true).bool());
        assert!(!Value::Bool(false).bool());
        assert!(ext(Value::Bool(true)).bool());
        assert!(!ext(Value::Bool(false)).bool());
        assert!(!Value::I32(1).bool());
        assert!(!Value::String("true".into()).bool());
        assert!(!Value::Null.bool());

        assert_eq!(Value::Binary(vec![1, 2, 3]).as_binary(), vec![1, 2, 3]);
        assert_eq!(Value::String("x".into()).as_binary(), Vec::<u8>::new());
        assert_eq!(Value::Null.as_binary(), Vec::<u8>::new());
        assert_eq!(ext(Value::Binary(vec![9])).as_binary(), Vec::<u8>::new());
    }

    #[test]
    fn test_asproxy_every_value_variant() {
        // Call every AsProxy method on every Value variant once.
        let variants = vec![
            Value::Null,
            Value::Bool(true),
            Value::I32(-3),
            Value::I64(-4),
            Value::U32(5),
            Value::U64(6),
            Value::F32(7.5),
            Value::F64(8.5),
            Value::String("9".into()),
            Value::Binary(vec![1, 2]),
            Value::Array(vec![Value::I32(1)]),
            one_entry_map(Value::String("k".into()), Value::I32(1)),
            ext(Value::I64(10)),
        ];
        for v in &variants {
            let _ = v.i32();
            let _ = v.i64();
            let _ = v.u32();
            let _ = v.u64();
            let _ = v.f64();
            let _ = v.usize();
            let _ = v.bool();
            let _ = v.string();
            let _ = v.as_binary();
        }
        assert_eq!(variants[0].i32(), 0);
        assert_eq!(variants[2].i32(), -3);
        assert_eq!(variants[7].f64(), 8.5);
        assert_eq!(variants[12].i64(), 10);
        assert_eq!(variants[9].as_binary(), vec![1, 2]);
    }

    // ------------------------------------------------------------------
    // ops.rs: AsProxy for bool / String / str / numeric macro types
    // ------------------------------------------------------------------

    #[test]
    fn test_asproxy_bool_primitives() {
        assert_eq!(true.i32(), 1);
        assert_eq!(false.i32(), 0);
        assert_eq!(true.i64(), 1);
        assert_eq!(false.i64(), 0);
        assert_eq!(true.u32(), 1);
        assert_eq!(false.u32(), 0);
        assert_eq!(true.u64(), 1);
        assert_eq!(false.u64(), 0);
        assert_eq!(true.usize(), 1);
        assert_eq!(false.usize(), 0);
        assert_eq!(true.f64(), 1.0);
        assert_eq!(false.f64(), 0.0);
        assert!(true.bool());
        assert!(!false.bool());
        assert_eq!(true.string(), "true");
        assert_eq!(false.string(), "false");
        assert_eq!(true.as_binary(), vec![1u8]);
        assert_eq!(false.as_binary(), vec![0u8]);
    }

    #[test]
    fn test_asproxy_string_primitives() {
        let s = String::from("123");
        assert_eq!(s.i32(), 123);
        assert_eq!(s.i64(), 123);
        assert_eq!(s.u32(), 123);
        assert_eq!(s.u64(), 123);
        assert_eq!(s.usize(), 123);
        assert_eq!(String::from("1.5").f64(), 1.5);
        assert!(String::from("true").bool());
        assert!(!String::from("false").bool());
        assert_eq!(s.string(), "123");
        assert_eq!(s.as_binary(), b"123".to_vec());

        // parse failures -> unwrap_or_default()
        let bad = String::from("abc");
        assert_eq!(bad.i32(), 0);
        assert_eq!(bad.i64(), 0);
        assert_eq!(bad.u32(), 0);
        assert_eq!(bad.u64(), 0);
        assert_eq!(bad.usize(), 0);
        assert_eq!(bad.f64(), 0.0);
        assert!(!bad.bool());
        assert_eq!(String::new().bool(), false);
    }

    #[test]
    fn test_asproxy_str_primitives() {
        assert_eq!("123".i32(), 123);
        assert_eq!("123".i64(), 123);
        assert_eq!("123".u32(), 123);
        assert_eq!("123".u64(), 123);
        assert_eq!("123".usize(), 123);
        assert_eq!("1.5".f64(), 1.5);
        assert!("true".bool());
        assert!(!("abc".bool()));
        assert_eq!("abc".string(), "abc");
        assert_eq!("abc".as_binary(), b"abc".to_vec());
        // parse failure paths
        assert_eq!("abc".i32(), 0);
        assert_eq!("abc".i64(), 0);
        assert_eq!("abc".u32(), 0);
        assert_eq!("abc".u64(), 0);
        assert_eq!("abc".usize(), 0);
        assert_eq!("abc".f64(), 0.0);
    }

    macro_rules! check_num_asproxy {
        ($v:expr) => {{
            let x = $v;
            assert_eq!(AsProxy::i32(&x), x as i32);
            assert_eq!(AsProxy::i64(&x), x as i64);
            assert_eq!(AsProxy::u32(&x), x as u32);
            assert_eq!(AsProxy::u64(&x), x as u64);
            assert_eq!(AsProxy::usize(&x), x as usize);
            assert_eq!(AsProxy::f64(&x), x as f64);
            assert_eq!(AsProxy::string(&x), x.to_string());
            assert_eq!(AsProxy::as_binary(&x), x.to_string().into_bytes());
        }};
    }

    macro_rules! check_num_truth {
        ($one:expr, $other:expr) => {{
            assert!(AsProxy::bool(&$one));
            assert!(!AsProxy::bool(&$other));
        }};
    }

    #[test]
    fn test_asproxy_number_macro_types() {
        // every type instantiated by impl_as_proxy_number!
        check_num_asproxy!(5i8);
        check_num_asproxy!(5i16);
        check_num_asproxy!(5i32);
        check_num_asproxy!(5i64);
        check_num_asproxy!(5isize);
        check_num_asproxy!(5u8);
        check_num_asproxy!(5u16);
        check_num_asproxy!(5u32);
        check_num_asproxy!(5u64);
        check_num_asproxy!(5usize);
        check_num_asproxy!(5.5f32);
        check_num_asproxy!(5.5f64);
    }

    #[test]
    fn test_asproxy_number_truthiness() {
        check_num_truth!(1i8, 2i8);
        check_num_truth!(1i16, 0i16);
        check_num_truth!(1i32, -1i32);
        check_num_truth!(1i64, 2i64);
        check_num_truth!(1isize, 0isize);
        check_num_truth!(1u8, 2u8);
        check_num_truth!(1u16, 0u16);
        check_num_truth!(1u32, 2u32);
        check_num_truth!(1u64, 0u64);
        check_num_truth!(1usize, 2usize);
        check_num_truth!(1.0f32, 2.0f32);
        check_num_truth!(1.0f64, 0.0f64);
    }

    // ------------------------------------------------------------------
    // ops.rs default method bodies (op_ne, op_lt, op_le, op_gt, op_ge)
    // ------------------------------------------------------------------

    #[test]
    fn test_eq_default_op_ne() {
        assert!(!true.op_ne(&true));
        assert!(true.op_ne(&false));
        assert!(!1i32.op_ne(&1i32));
        assert!(1i32.op_ne(&2i32));
        assert!(!Value::I32(1).op_ne(&Value::I32(1)));
        assert!(Value::I32(1).op_ne(&Value::I32(2)));
        assert!(1i32.op_ne(&2i64));
        assert!(!Value::String("a".into()).op_ne(&Value::String("a".into())));
    }

    #[test]
    fn test_ord_default_methods() {
        // primitive cmp_i128 path
        assert!(1i32.op_lt(&2i32));
        assert!(!2i32.op_lt(&1i32));
        assert!(2i32.op_le(&2i32)); // Some(Equal) arm of op_le
        assert!(1i32.op_le(&2i32));
        assert!(!3i32.op_le(&2i32));
        assert!(3i32.op_gt(&2i32));
        assert!(!2i32.op_gt(&2i32));
        assert!(2i32.op_ge(&2i32)); // Some(Equal) arm of op_ge
        assert!(3i32.op_ge(&2i32));
        assert!(!1i32.op_ge(&2i32));

        // incomparable floats -> None -> false for every method
        assert!(!f64::NAN.op_lt(&1.0));
        assert!(!f64::NAN.op_le(&1.0));
        assert!(!f64::NAN.op_gt(&1.0));
        assert!(!f64::NAN.op_ge(&1.0));
        assert!(!1.0f64.op_lt(&f64::NAN));
        assert!(!1.0f64.op_ge(&f64::NAN));

        // Value incomparable (Null vs number) -> false for every method
        assert!(!Value::Null.op_lt(&Value::I32(1)));
        assert!(!Value::Null.op_le(&Value::I32(1)));
        assert!(!Value::Null.op_gt(&Value::I32(1)));
        assert!(!Value::Null.op_ge(&Value::I32(1)));

        // Value comparable
        assert!(Value::I32(1).op_lt(&Value::I32(2)));
        assert!(Value::I32(2).op_le(&Value::I32(2)));
        assert!(Value::I32(3).op_gt(&Value::I32(2)));
        assert!(Value::I32(2).op_ge(&Value::I32(2)));

        // bool / str paths
        assert!(false.op_lt(&true));
        assert!(true.op_le(&true));
        assert!(true.op_gt(&false));
        assert!(true.op_ge(&true));
        assert!("a".op_lt(&"b"));
    }

    // ------------------------------------------------------------------
    // ops_eq.rs
    // ------------------------------------------------------------------

    #[test]
    fn test_eq_value_value_all_combos() {
        let a = Value::I32(1);
        let b = Value::I32(1);
        let c = Value::I32(2);
        let ra = &a;
        let rra = &ra;

        // Value / Value
        assert!(a.op_eq(&b));
        // Value / &Value
        assert!(a.op_eq(&&b));
        // Value / &&Value
        assert!(a.op_eq(&&&b));
        // &Value / Value
        assert!(ra.op_eq(&b));
        // &Value / &Value
        assert!(ra.op_eq(&&b));
        // &Value / &&Value
        assert!(ra.op_eq(&&&b));
        // &&Value / Value
        assert!(rra.op_eq(&b));
        // &&Value / &Value
        assert!(rra.op_eq(&&b));
        // &&Value / &&Value
        assert!(rra.op_eq(&&&b));

        // and all nine on unequal values
        assert!(!a.op_eq(&c));
        assert!(!a.op_eq(&&c));
        assert!(!a.op_eq(&&&c));
        assert!(!ra.op_eq(&c));
        assert!(!ra.op_eq(&&c));
        assert!(!ra.op_eq(&&&c));
        assert!(!rra.op_eq(&c));
        assert!(!rra.op_eq(&&c));
        assert!(!rra.op_eq(&&&c));
    }

    #[test]
    fn test_eq_value_cross_kinds() {
        // integer cross-kind path (i128 comparison)
        assert!(Value::I32(1).op_eq(&Value::U64(1)));
        assert!(Value::I64(1).op_eq(&Value::U32(1)));
        assert!(!Value::U64(u64::MAX).op_eq(&Value::I64(-1)));
        // integer vs float goes through the floating path
        assert!(Value::I32(1).op_eq(&Value::F64(1.0)));
        assert!(Value::F32(1.0).op_eq(&Value::F64(1.0)));
        assert!(!Value::F32(0.1).op_eq(&Value::F64(0.1)));
        // NaN never equals itself
        assert!(!Value::F64(f64::NAN).op_eq(&Value::F64(f64::NAN)));
        // Null / Bool / String / Array / Map / Binary fallback equality
        assert!(Value::Null.op_eq(&Value::Null));
        assert!(!Value::Null.op_eq(&Value::Bool(false)));
        assert!(Value::Bool(true).op_eq(&Value::Bool(true)));
        assert!(!Value::Bool(true).op_eq(&Value::Bool(false)));
        assert!(Value::String("x".into()).op_eq(&Value::String("x".into())));
        assert!(!Value::String("x".into()).op_eq(&Value::I32(1)));
        assert!(Value::Array(vec![Value::I32(1)]).op_eq(&Value::Array(vec![Value::I32(1)])));
        assert!(!Value::Array(vec![Value::I32(1)]).op_eq(&Value::Array(vec![])));
        assert!(one_entry_map(Value::String("k".into()), Value::I32(1))
            .op_eq(&one_entry_map(Value::String("k".into()), Value::I32(1))));
        assert!(Value::Binary(vec![1, 2]).op_eq(&Value::Binary(vec![1, 2])));
        // Ext is stripped before comparing
        assert!(ext(Value::I32(1)).op_eq(&Value::I32(1)));
        assert!(ext(ext(Value::I32(1))).op_eq(&Value::I32(1)));
        assert!(ext(Value::F64(2.0)).op_eq(&Value::F64(2.0)));
    }

    #[test]
    fn test_eq_value_scalar_all_types() {
        eq_scalar_all!(Value::U64(5), 5u8);
        eq_scalar_all!(Value::U64(5), 5u16);
        eq_scalar_all!(Value::U64(5), 5u32);
        eq_scalar_all!(Value::U64(5), 5u64);
        eq_scalar_all!(Value::I64(5), 5i8);
        eq_scalar_all!(Value::I64(5), 5i16);
        eq_scalar_all!(Value::I64(5), 5i32);
        eq_scalar_all!(Value::I64(5), 5i64);
        eq_scalar_all!(Value::I64(5), 5isize);
        eq_scalar_all!(Value::I64(5), 5usize);
        eq_scalar_all!(Value::F64(5.0), 5.0f32);
        eq_scalar_all!(Value::F32(5.0), 5.0f64);
        eq_scalar_all!(Value::Bool(true), true);
    }

    #[test]
    fn test_eq_scalar_mismatches() {
        // false arm of each value_eq_* helper
        assert!(!Value::U64(5).op_eq(&6u64));
        assert!(!Value::I64(5).op_eq(&6i64));
        assert!(!Value::F64(5.0).op_eq(&6.0f64));
        assert!(!Value::Bool(true).op_eq(&false));
        assert!(!Value::String("5".into()).op_eq(&5u64));
        assert!(!Value::Null.op_eq(&5u64));
        assert!(!Value::I32(5).op_eq(&true));
        assert_eq!(Value::I32(5).op_ne(&5u64), false);
    }

    #[test]
    fn test_eq_value_str_impls() {
        let v = Value::String("abc".into());
        let n = Value::I32(1);

        // Value: PartialEq<str>, PartialEq<&str>
        assert!(v.op_eq("abc"));
        assert!(!v.op_eq("abd"));
        assert!(v.op_eq(&"abc"));
        assert!(!v.op_eq(&"abd"));
        // non-string Value renders as empty string for as_str()
        assert!(!n.op_eq("1"));

        // str / &str against Value (UFCS to pin the impls)
        assert!(<str as OpEq<Value>>::op_eq("abc", &v));
        assert!(!<str as OpEq<Value>>::op_eq("abc", &n));
        assert!(<&str as OpEq<Value>>::op_eq(&"abc", &v));
        // PartialEq<&str> for str
        assert!(<str as OpEq<&str>>::op_eq("abc", &"abc"));
        assert!(!<str as OpEq<&str>>::op_eq("abc", &"abd"));
    }

    #[test]
    fn test_eq_value_string_impls() {
        let v = Value::String("abc".into());
        let s = String::from("abc");
        let bad = String::from("abd");
        let rs = &s;

        // Value / String, Value / &String
        assert!(v.op_eq(&s));
        assert!(!v.op_eq(&bad));
        assert!(v.op_eq(&&s));
        assert!(!v.op_eq(&&bad));
        // &Value / String, &Value / &String, &Value / &str
        let rv = &v;
        assert!(rv.op_eq(&s));
        assert!(rv.op_eq(&&s));
        assert!(rv.op_eq(&"abc"));
        // String / Value, &String / Value
        assert!(s.op_eq(&v));
        assert!(rs.op_eq(&v));
        assert!(!bad.op_eq(&v));
    }

    #[test]
    fn test_eq_cow() {
        let v = Value::I32(1);
        let w = Value::I32(1);
        let x = Value::I32(2);
        assert!(v.op_eq(&Cow::Borrowed(&w)));
        assert!(!v.op_eq(&Cow::Borrowed(&x)));
        assert!(v.op_eq(&Cow::Owned(w.clone())));
        assert!(!v.op_eq(&Cow::Owned(x)));
    }

    #[test]
    fn test_eq_same_type_bool_string_str() {
        // bool
        assert!(true.op_eq(&true));
        assert!(!true.op_eq(&false));
        assert!(true.op_eq(&&true)); // bool / &bool
        let t = true;
        let f = false;
        let rt = &t;
        assert!(rt.op_eq(&t)); // &bool / bool
        assert!(rt.op_eq(&&t)); // &bool / &bool
        assert!(!rt.op_eq(&&f));

        // String
        let a = String::from("a");
        let b = String::from("a");
        let c = String::from("c");
        let ra = &a;
        assert!(a.op_eq(&b));
        assert!(!a.op_eq(&c));
        assert!(a.op_eq(&&b));
        assert!(ra.op_eq(&b));
        assert!(ra.op_eq(&&b));
        assert!(!ra.op_eq(&&c));

        // &str (UFCS; four generated impls)
        let x: &str = "x";
        let y: &str = "y";
        let rx: &&str = &x;
        assert!(<&str as OpEq<&str>>::op_eq(&x, &x));
        assert!(!<&str as OpEq<&str>>::op_eq(&x, &y));
        assert!(<&str as OpEq<&&str>>::op_eq(&x, &&x));
        assert!(<&&str as OpEq<&str>>::op_eq(&rx, &x));
        assert!(<&&str as OpEq<&&str>>::op_eq(&rx, &&x));
    }

    #[test]
    fn test_eq_str_scalar_matrix() {
        // impl_str_eq! for unsigned: eq_str_u64
        assert!(<&str as OpEq<u8>>::op_eq(&"5", &5u8));
        assert!(<u8 as OpEq<&str>>::op_eq(&5u8, &"5"));
        assert!(<u8 as OpEq<&&str>>::op_eq(&5u8, &&"5"));
        assert!(<u8 as OpEq<&&&str>>::op_eq(&5u8, &&&"5"));
        assert!(<&&str as OpEq<u8>>::op_eq(&&"5", &5u8));
        assert!(<&str as OpEq<u16>>::op_eq(&"5", &5u16));
        assert!(<&str as OpEq<u32>>::op_eq(&"5", &5u32));
        assert!(<&str as OpEq<u64>>::op_eq(&"5", &5u64));

        // signed: eq_str_i64
        assert!(<&str as OpEq<i8>>::op_eq(&"5", &5i8));
        assert!(<&str as OpEq<i16>>::op_eq(&"5", &5i16));
        assert!(<&str as OpEq<i32>>::op_eq(&"5", &5i32));
        assert!(<&str as OpEq<i64>>::op_eq(&"5", &5i64));
        assert!(<&str as OpEq<isize>>::op_eq(&"5", &5isize));
        assert!(<&str as OpEq<usize>>::op_eq(&"5", &5usize));

        // float: eq_str_f64
        assert!(<&str as OpEq<f32>>::op_eq(&"5", &5.0f32));
        assert!(<&str as OpEq<f64>>::op_eq(&"5", &5.0f64));

        // bool: eq_str_bool covers "true", "false" and the fallthrough arm
        assert!(<&str as OpEq<bool>>::op_eq(&"true", &true));
        assert!(<&str as OpEq<bool>>::op_eq(&"false", &false));
        assert!(!<&str as OpEq<bool>>::op_eq(&"false", &true));
        assert!(!<&str as OpEq<bool>>::op_eq(&"true", &false));
        assert!(!<&str as OpEq<bool>>::op_eq(&"other", &true));
        // mismatch path of the numeric helpers
        assert!(!<&str as OpEq<u64>>::op_eq(&"6", &5u64));
        assert!(!<u64 as OpEq<&str>>::op_eq(&5u64, &"6"));

        // all five generated impls for every scalar type in impl_str_eq!
        str_eq_all!(u8, "5", 5u8);
        str_eq_all!(u16, "5", 5u16);
        str_eq_all!(u32, "5", 5u32);
        str_eq_all!(u64, "5", 5u64);
        str_eq_all!(i8, "5", 5i8);
        str_eq_all!(i16, "5", 5i16);
        str_eq_all!(i32, "5", 5i32);
        str_eq_all!(i64, "5", 5i64);
        str_eq_all!(isize, "5", 5isize);
        str_eq_all!(usize, "5", 5usize);
        str_eq_all!(f32, "5", 5.0f32);
        str_eq_all!(f64, "5", 5.0f64);
        str_eq_all!(bool, "true", true);
    }

    #[test]
    fn test_eq_compare_matrices() {
        // keys ints => rows ints
        eq_matrix!([1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize];
                   [1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize]);
        // keys ints => rows floats (eq_f64 cast)
        eq_matrix!([1.0f32, 1.0f64];
                   [1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize]);
        // keys floats => rows ints
        eq_matrix!([1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize];
                   [1.0f32, 1.0f64]);
        // keys floats => rows floats
        eq_matrix!([1.0f32, 1.0f64]; [1.0f32, 1.0f64]);
    }

    // ------------------------------------------------------------------
    // ops_cmp.rs
    // ------------------------------------------------------------------

    #[test]
    fn test_ord_value_value_all_combos() {
        let a = Value::I32(1);
        let b = Value::I32(2);
        let c = Value::I32(2);
        let ra = &a;
        let rra = &ra;

        assert_eq!(a.op_partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(a.op_partial_cmp(&&b), Some(Ordering::Less));
        assert_eq!(a.op_partial_cmp(&&&b), Some(Ordering::Less));
        assert_eq!(ra.op_partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(ra.op_partial_cmp(&&b), Some(Ordering::Less));
        assert_eq!(ra.op_partial_cmp(&&&b), Some(Ordering::Less));
        assert_eq!(rra.op_partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(rra.op_partial_cmp(&&b), Some(Ordering::Less));
        assert_eq!(rra.op_partial_cmp(&&&b), Some(Ordering::Less));

        assert_eq!(b.op_partial_cmp(&c), Some(Ordering::Equal));
        assert_eq!(b.op_partial_cmp(&a), Some(Ordering::Greater));

        // incomparable values
        assert_eq!(Value::Null.op_partial_cmp(&Value::I32(1)), None);
        assert_eq!(Value::String("a".into()).op_partial_cmp(&Value::I32(1)), None);
        assert_eq!(
            Value::Bool(true).op_partial_cmp(&Value::Bool(false)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            Value::String("a".into()).op_partial_cmp(&Value::String("b".into())),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn test_ord_value_scalar_all_types() {
        cmp_scalar_all!(Value::U64(5), 5u8);
        cmp_scalar_all!(Value::U64(5), 5u16);
        cmp_scalar_all!(Value::U64(5), 5u32);
        cmp_scalar_all!(Value::U64(5), 5u64);
        cmp_scalar_all!(Value::I64(5), 5i8);
        cmp_scalar_all!(Value::I64(5), 5i16);
        cmp_scalar_all!(Value::I64(5), 5i32);
        cmp_scalar_all!(Value::I64(5), 5i64);
        cmp_scalar_all!(Value::I64(5), 5isize);
        cmp_scalar_all!(Value::I64(5), 5usize);
        cmp_scalar_all!(Value::F64(5.0), 5.0f32);
        cmp_scalar_all!(Value::F32(5.0), 5.0f64);
        cmp_scalar_all!(Value::Bool(true), true);
    }

    #[test]
    fn test_ord_scalar_reverse_orderings() {
        // reverse helper maps Less -> Greater etc.
        assert_eq!(Value::U64(5).op_partial_cmp(&6u64), Some(Ordering::Less));
        assert_eq!(Value::U64(5).op_partial_cmp(&4u64), Some(Ordering::Greater));
        assert_eq!(5u64.op_partial_cmp(&Value::U64(6)), Some(Ordering::Less));
        assert_eq!(5u64.op_partial_cmp(&Value::U64(4)), Some(Ordering::Greater));
        assert_eq!(5i64.op_partial_cmp(&Value::I64(6)), Some(Ordering::Less));
        assert_eq!(5i64.op_partial_cmp(&Value::I64(4)), Some(Ordering::Greater));
        assert_eq!(5.0f64.op_partial_cmp(&Value::F64(6.0)), Some(Ordering::Less));
        assert_eq!(5.0f64.op_partial_cmp(&Value::F64(4.0)), Some(Ordering::Greater));
        assert_eq!(false.op_partial_cmp(&Value::Bool(true)), Some(Ordering::Less));
        assert_eq!(true.op_partial_cmp(&Value::Bool(false)), Some(Ordering::Greater));

        // reverse helper on None stays None
        assert_eq!(5u64.op_partial_cmp(&Value::String("x".into())), None);
        assert_eq!(5i64.op_partial_cmp(&Value::String("x".into())), None);
        assert_eq!(5.0f64.op_partial_cmp(&Value::String("x".into())), None);
        assert_eq!(true.op_partial_cmp(&Value::I32(1)), None);
        assert_eq!(Value::String("5".into()).op_partial_cmp(&5u64), None);
    }

    #[test]
    fn test_ord_value_str_impls() {
        let v = Value::String("b".into());
        let a = Value::String("a".into());
        let rv = &v;
        let rrv = &rv;

        // Value / &str, Value / &&str
        assert_eq!(v.op_partial_cmp(&"a"), Some(Ordering::Greater));
        assert_eq!(v.op_partial_cmp(&&"a"), Some(Ordering::Greater));
        assert_eq!(a.op_partial_cmp(&"b"), Some(Ordering::Less));
        // &Value and &&Value receivers
        assert_eq!(rv.op_partial_cmp(&"a"), Some(Ordering::Greater));
        assert_eq!(rv.op_partial_cmp(&&"a"), Some(Ordering::Greater));
        assert_eq!(rrv.op_partial_cmp(&"a"), Some(Ordering::Greater));
        assert_eq!(rrv.op_partial_cmp(&&"a"), Some(Ordering::Greater));
        // non-string value renders via AsProxy::string()
        assert_eq!(Value::I32(2).op_partial_cmp(&"1"), Some(Ordering::Greater));

        // &str / &&str receivers against Value (UFCS keeps impls unambiguous)
        let x: &str = "a";
        let rx: &&str = &x;
        assert_eq!(
            <&str as OpOrd<Value>>::op_partial_cmp(&x, &v),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&str as OpOrd<&Value>>::op_partial_cmp(&x, &&v),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&str as OpOrd<&&Value>>::op_partial_cmp(&x, &&&v),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&&str as OpOrd<Value>>::op_partial_cmp(&rx, &v),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&&str as OpOrd<&Value>>::op_partial_cmp(&rx, &&v),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&&str as OpOrd<&&Value>>::op_partial_cmp(&rx, &&&v),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn test_ord_string_impls() {
        let s = String::from("a");
        let t = String::from("b");
        let rs = &s;
        let rt = &t;
        let rrs = &rs;
        let rrt = &rt;
        let x: &str = "a";
        let rx: &&str = &x;

        // &str vs &str
        assert_eq!(
            <&str as OpOrd<&str>>::op_partial_cmp(&x, &"a"),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&str as OpOrd<&str>>::op_partial_cmp(&x, &"b"),
            Some(Ordering::Less)
        );

        // String receivers
        assert_eq!(
            <String as OpOrd<&str>>::op_partial_cmp(&s, &x),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <String as OpOrd<String>>::op_partial_cmp(&s, &t),
            Some(Ordering::Less)
        );
        assert_eq!(
            <String as OpOrd<&String>>::op_partial_cmp(&s, &rt),
            Some(Ordering::Less)
        );
        assert_eq!(
            <String as OpOrd<&&String>>::op_partial_cmp(&s, &rrt),
            Some(Ordering::Less)
        );

        // &String receivers
        assert_eq!(
            <&String as OpOrd<&str>>::op_partial_cmp(&rs, &x),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&String as OpOrd<&&str>>::op_partial_cmp(&rs, &&x),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&String as OpOrd<&&&str>>::op_partial_cmp(&rs, &&&x),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&String as OpOrd<String>>::op_partial_cmp(&rs, &t),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&String as OpOrd<&String>>::op_partial_cmp(&rs, &rt),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&String as OpOrd<&&String>>::op_partial_cmp(&rs, &rrt),
            Some(Ordering::Less)
        );

        // &&String receivers
        assert_eq!(
            <&&String as OpOrd<String>>::op_partial_cmp(&rrs, &t),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&&String as OpOrd<&String>>::op_partial_cmp(&rrs, &rt),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&&String as OpOrd<&&String>>::op_partial_cmp(&rrs, &rrt),
            Some(Ordering::Less)
        );
        let _ = rx;
    }

    #[test]
    fn test_ord_bool_same_type() {
        let t = true;
        let f = false;
        let rt = &t;
        let rf = &f;
        assert_eq!(
            <bool as OpOrd<bool>>::op_partial_cmp(&t, &f),
            Some(Ordering::Greater)
        );
        assert_eq!(
            <bool as OpOrd<bool>>::op_partial_cmp(&f, &t),
            Some(Ordering::Less)
        );
        assert_eq!(
            <bool as OpOrd<&bool>>::op_partial_cmp(&t, &&f),
            Some(Ordering::Greater)
        );
        assert_eq!(
            <&bool as OpOrd<bool>>::op_partial_cmp(&rt, &f),
            Some(Ordering::Greater)
        );
        assert_eq!(
            <&bool as OpOrd<&bool>>::op_partial_cmp(&rt, &rf),
            Some(Ordering::Greater)
        );
        assert_eq!(
            <&bool as OpOrd<&bool>>::op_partial_cmp(&rt, &&t),
            Some(Ordering::Equal)
        );
    }

    #[test]
    fn test_ord_compare_matrices() {
        // keys ints => rows ints (cmp_i128)
        cmp_matrix!([1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize];
                    [1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize]);
        // keys ints => rows floats (cmp_f64)
        cmp_matrix!([1.0f32, 1.0f64];
                    [1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize]);
        // keys floats => rows ints
        cmp_matrix!([1u8, 1u16, 1u32, 1u64, 1usize, 1i8, 1i16, 1i32, 1i64, 1isize];
                    [1.0f32, 1.0f64]);
        // keys floats => rows floats
        cmp_matrix!([1.0f32, 1.0f64]; [1.0f32, 1.0f64]);
    }

    // ------------------------------------------------------------------
    // ops_compare.rs helpers (reached through ops_eq / ops_cmp)
    // ------------------------------------------------------------------

    #[test]
    fn test_compare_value_partial_cmp_edges() {
        // Null/Null, Bool/Bool, String/String arms
        assert_eq!(
            Value::Null.op_partial_cmp(&Value::Null),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::Bool(false).op_partial_cmp(&Value::Bool(true)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Value::String("b".into()).op_partial_cmp(&Value::String("a".into())),
            Some(Ordering::Greater)
        );
        // integer path crossing Int/UInt widths
        assert_eq!(
            Value::I32(1).op_partial_cmp(&Value::U64(2)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Value::U32(3).op_partial_cmp(&Value::I64(3)),
            Some(Ordering::Equal)
        );
        // floating path when one side is a float
        assert_eq!(
            Value::F32(1.5).op_partial_cmp(&Value::F64(2.5)),
            Some(Ordering::Less)
        );
        assert_eq!(
            Value::F64(2.5).op_partial_cmp(&Value::I32(2)),
            Some(Ordering::Greater)
        );
        // NaN -> None
        assert_eq!(
            Value::F64(f64::NAN).op_partial_cmp(&Value::F64(1.0)),
            None
        );
        // incomparable kinds -> None
        assert_eq!(
            Value::Bool(true).op_partial_cmp(&Value::I32(1)),
            None
        );
        assert_eq!(
            Value::String("1".into()).op_partial_cmp(&Value::I32(1)),
            None
        );
        assert_eq!(
            Value::Array(vec![]).op_partial_cmp(&Value::Array(vec![])),
            None
        );
        assert_eq!(
            Value::Map(ValueMap::new()).op_partial_cmp(&Value::Map(ValueMap::new())),
            None
        );
        assert_eq!(
            Value::Binary(vec![1]).op_partial_cmp(&Value::Binary(vec![1])),
            None
        );
        // Ext is stripped before comparing
        assert_eq!(
            ext(ext(Value::I32(1))).op_partial_cmp(&Value::I32(1)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::Ext("x", Box::new(Value::F32(1.5))).op_partial_cmp(&Value::F64(1.5)),
            Some(Ordering::Equal)
        );
    }

    #[test]
    fn test_compare_value_eq_edges() {
        // integer path
        assert!(Value::I32(2).op_eq(&Value::U64(2)));
        assert!(!Value::I32(2).op_eq(&Value::U64(3)));
        // floating path
        assert!(Value::F32(2.0).op_eq(&Value::F64(2.0)));
        assert!(!Value::F32(2.0).op_eq(&Value::F64(3.0)));
        // fallback equality for all non-numeric kinds
        assert!(Value::Null.op_eq(&Value::Null));
        assert!(Value::Bool(false).op_eq(&Value::Bool(false)));
        assert!(Value::String("z".into()).op_eq(&Value::String("z".into())));
        assert!(Value::Array(vec![Value::Null]).op_eq(&Value::Array(vec![Value::Null])));
        assert!(one_entry_map(Value::I32(1), Value::Null)
            .op_eq(&one_entry_map(Value::I32(1), Value::Null)));
        assert!(Value::Binary(vec![0, 1]).op_eq(&Value::Binary(vec![0, 1])));
        assert!(!Value::Binary(vec![0]).op_eq(&Value::Binary(vec![1])));
        // Ext stripping then fallback equality
        assert!(ext(Value::String("z".into())).op_eq(&Value::String("z".into())));
        assert!(ext(Value::Bool(true)).op_eq(&Value::Bool(true)));
    }

    #[test]
    fn test_compare_scalar_helper_arms() {
        // value_cmp_i128 / value_cmp_f64_inner arms for every numeric Value kind
        assert_eq!(
            Value::I32(5).op_partial_cmp(&5u64),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::I64(5).op_partial_cmp(&5u8),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::U32(5).op_partial_cmp(&5i8),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::U64(5).op_partial_cmp(&5i16),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::F32(5.0).op_partial_cmp(&5u16),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::F64(5.0).op_partial_cmp(&5u32),
            Some(Ordering::Equal)
        );
        // the `_ => None` arm of value_cmp_i128
        assert_eq!(Value::String("5".into()).op_partial_cmp(&5u64), None);
        assert_eq!(Value::Bool(true).op_partial_cmp(&5u64), None);
        assert_eq!(Value::Null.op_partial_cmp(&5u64), None);
        assert_eq!(Value::Array(vec![]).op_partial_cmp(&5u64), None);
        assert_eq!(
            Value::Map(ValueMap::new()).op_partial_cmp(&5u64),
            None
        );
        assert_eq!(Value::Binary(vec![]).op_partial_cmp(&5u64), None);

        // value_cmp_f64_inner (floating_value arms)
        assert_eq!(5.0f64.op_partial_cmp(&Value::I32(5)), Some(Ordering::Equal));
        assert_eq!(5.0f64.op_partial_cmp(&Value::I64(5)), Some(Ordering::Equal));
        assert_eq!(5.0f64.op_partial_cmp(&Value::U32(5)), Some(Ordering::Equal));
        assert_eq!(5.0f64.op_partial_cmp(&Value::U64(5)), Some(Ordering::Equal));
        assert_eq!(5.0f64.op_partial_cmp(&Value::F32(5.0)), Some(Ordering::Equal));
        assert_eq!(5.0f64.op_partial_cmp(&Value::F64(5.0)), Some(Ordering::Equal));
        // `_ => None` arm of floating_value
        assert_eq!(5.0f64.op_partial_cmp(&Value::String("5".into())), None);
        assert_eq!(5.0f64.op_partial_cmp(&Value::Null), None);
        assert_eq!(5.0f64.op_partial_cmp(&Value::Bool(true)), None);
        assert_eq!(5.0f64.op_partial_cmp(&Value::Array(vec![])), None);
        assert_eq!(5.0f64.op_partial_cmp(&Value::Binary(vec![])), None);
        assert_eq!(5.0f64.op_partial_cmp(&Value::Map(ValueMap::new())), None);
        // NaN from the Value side
        assert_eq!(
            5.0f64.op_partial_cmp(&Value::F64(f64::NAN)),
            None
        );

        // value_cmp_bool_inner arms
        assert_eq!(
            Value::Bool(true).op_partial_cmp(&true),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Value::Bool(false).op_partial_cmp(&true),
            Some(Ordering::Less)
        );
        // `_ => None` arm
        assert_eq!(Value::String("true".into()).op_partial_cmp(&true), None);
        assert_eq!(Value::I32(1).op_partial_cmp(&true), None);
    }

    // ------------------------------------------------------------------
    // from_bool.rs
    // ------------------------------------------------------------------

    #[test]
    fn test_from_bool_all_impls() {
        assert!(bool::op_from(true));
        assert!(!bool::op_from(false));
        assert!(bool::op_from(&true));
        assert!(!bool::op_from(&false));
        assert!(bool::op_from(&&true));
        assert!(!bool::op_from(&&false));

        // Value sources delegate to AsProxy::bool()
        assert!(bool::op_from(Value::Bool(true)));
        assert!(!bool::op_from(Value::Bool(false)));
        assert!(bool::op_from(&Value::Bool(true)));
        assert!(!bool::op_from(&Value::Bool(false)));
        assert!(bool::op_from(&&Value::Bool(true)));
        assert!(!bool::op_from(&&Value::Bool(false)));
        // non-bool values -> false (as_bool is None)
        assert!(!bool::op_from(Value::I32(1)));
        assert!(!bool::op_from(Value::U64(1)));
        assert!(!bool::op_from(Value::String("true".into())));
        assert!(!bool::op_from(Value::Null));
        assert!(!bool::op_from(&Value::Array(vec![])));
        // Ext delegates to the inner value
        assert!(bool::op_from(ext(Value::Bool(true))));
        assert!(!bool::op_from(ext(Value::Bool(false))));
        assert!(!bool::op_from(ext(Value::I32(1))));
    }

    // ------------------------------------------------------------------
    // into_sql.rs
    // ------------------------------------------------------------------

    #[test]
    fn test_intosql_scalars() {
        assert_eq!(true.sql(), "true");
        assert_eq!(false.sql(), "false");
        assert_eq!(String::from("hello").sql(), "hello");
        assert_eq!("hello".sql(), "hello");
        assert_eq!(1i32.sql(), "1");
        assert_eq!((-2i64).sql(), "-2");
        assert_eq!(1.5f32.sql(), "1.5");
        assert_eq!(2.0f64.sql(), "2");
        assert_eq!(3u32.sql(), "3");
        assert_eq!(4u64.sql(), "4");
    }

    #[test]
    fn test_intosql_value_scalars() {
        assert_eq!(Value::String("x".into()).sql(), "'x'");
        assert_eq!(Value::Null.sql(), "null");
        assert_eq!(Value::Bool(true).sql(), "true");
        assert_eq!(Value::I32(1).sql(), "1");
        assert_eq!(Value::I64(-1).sql(), "-1");
        assert_eq!(Value::U32(2).sql(), "2");
        assert_eq!(Value::U64(3).sql(), "3");
        assert_eq!(Value::F32(1.5).sql(), "1.5");
        assert_eq!(Value::F64(2.0).sql(), "2");
        assert_eq!(Value::Binary(vec![1, 2]).sql(), "[1, 2]");
        // Ext over a string is quoted; Ext over a number renders as ""
        assert_eq!(ext(Value::String("y".into())).sql(), "'y'");
        assert_eq!(ext(Value::I32(3)).sql(), "");
    }

    #[test]
    fn test_intosql_value_array() {
        // string elements are quoted
        assert_eq!(
            Value::Array(vec![Value::String("a".into()), Value::String("b".into())]).sql(),
            "('a','b')"
        );
        // numeric elements are not
        assert_eq!(
            Value::Array(vec![Value::I32(1), Value::I32(2)]).sql(),
            "(1,2)"
        );
        // mixed elements
        assert_eq!(
            Value::Array(vec![Value::String("a".into()), Value::I32(1)]).sql(),
            "('a',1)"
        );
        // empty array avoids the `pop` branch
        assert_eq!(Value::Array(vec![]).sql(), "()");
    }

    #[test]
    fn test_intosql_value_map() {
        let key = || Value::String("a".into());
        // empty map
        assert_eq!(Value::Map(ValueMap::new()).sql(), "");
        // string value
        assert_eq!(
            one_entry_map(key(), Value::String("x".into())).sql(),
            "a'x' "
        );
        // numeric value
        assert_eq!(one_entry_map(key(), Value::I32(1)).sql(), "a1 ");
        // array value takes the `is_array` branch
        assert_eq!(
            one_entry_map(key(), Value::Array(vec![Value::String("x".into())])).sql(),
            "a('x') "
        );
        // nested map value falls back to `string()` (Display)
        assert_eq!(
            one_entry_map(key(), one_entry_map(Value::String("b".into()), Value::I32(2))).sql(),
            "a{\"b\":2} "
        );
        // non-string key
        assert_eq!(one_entry_map(Value::I32(7), Value::I32(8)).sql(), "78 ");
        assert_eq!(one_entry_map(Value::Bool(true), Value::Null).sql(), "truenull ");
        assert_eq!(
            one_entry_map(ext(Value::String("k".into())), Value::String("v".into())).sql(),
            "k'v' "
        );
    }

    #[test]
    fn test_intosql_ref_and_cow() {
        let v = Value::String("z".into());
        assert_eq!((&v).sql(), "'z'");
        assert_eq!(Cow::Borrowed(&v).sql(), "'z'");
        assert_eq!(Cow::<Value>::Owned(v.clone()).sql(), "'z'");

        let n = Value::Array(vec![Value::I32(1)]);
        assert_eq!((&n).sql(), "(1)");
        assert_eq!(Cow::Borrowed(&n).sql(), "(1)");
        assert_eq!(Cow::<Value>::Owned(n).sql(), "(1)");
    }

    // ------------------------------------------------------------------
    // error.rs
    // ------------------------------------------------------------------

    #[test]
    fn test_codegen_error_display_and_from() {
        let e: CodegenError = CodegenError::from("boom");
        assert_eq!(e.to_string(), "boom");
        assert_eq!(format!("{}", e), "boom");
        assert_eq!(format!("{:?}", e), "E(\"boom\")");

        let e2: CodegenError = "abc".into();
        assert_eq!(e2.to_string(), "abc");

        let e3: CodegenError = String::from("owned").into();
        assert_eq!(e3.to_string(), "owned");

        // From<syn::Error>
        let syn_err = syn::parse_str::<syn::Expr>("!!").err().unwrap();
        let syn_msg = syn_err.to_string();
        let e4 = CodegenError::from(syn_err);
        assert_eq!(e4.to_string(), syn_msg);

        // From<io::Error>
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "io boom");
        let e5 = CodegenError::from(io_err);
        assert_eq!(e5.to_string(), "io boom");

        // From<&dyn std::error::Error>
        let io_err2 = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let e6 = CodegenError::from(&io_err2 as &dyn StdError);
        assert_eq!(e6.to_string(), "missing");
    }

    #[test]
    fn test_codegen_error_clone() {
        let a = CodegenError::from("x");
        let b = a.clone();
        assert_eq!(a.to_string(), b.to_string());
        assert_eq!(b.to_string(), "x");

        let mut c = CodegenError::from("old");
        c.clone_from(&a);
        assert_eq!(c.to_string(), "x");
    }

    #[test]
    fn test_codegen_error_serde() {
        let e = CodegenError::from("boom");
        let json = serde_json::to_string(&e).unwrap();
        assert_eq!(json, "\"boom\"");

        // borrowed string -> visit_str (serde default visit_borrowed_str)
        let d: CodegenError = serde_json::from_str("\"boom\"").unwrap();
        assert_eq!(d.to_string(), "boom");
        // escaped string forces an owned allocation -> visit_string
        let d2: CodegenError = serde_json::from_str("\"bo\\\"om\"").unwrap();
        assert_eq!(d2.to_string(), "bo\"om");
        // owned Value input also reaches visit_string
        let d3: CodegenError =
            serde_json::from_value(serde_json::Value::String("owned".into())).unwrap();
        assert_eq!(d3.to_string(), "owned");

        // visitor.expecting() branch: a non-string JSON scalar is rejected
        let type_err = serde_json::from_str::<CodegenError>("123").unwrap_err();
        assert!(type_err.to_string().contains("a string"), "{}", type_err);
    }

    #[test]
    fn test_codegen_error_std_error_object() {
        let boxed: Box<dyn StdError> = Box::new(CodegenError::from("boxed"));
        assert_eq!(boxed.to_string(), "boxed");
        assert!(boxed.source().is_none());

        let source = CodegenError::from("ref");
        let dyn_ref: &dyn StdError = &source;
        assert_eq!(dyn_ref.to_string(), "ref");
        let converted = CodegenError::from(dyn_ref);
        assert_eq!(converted.to_string(), "ref");
    }
}
