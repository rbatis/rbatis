//! Runtime coverage for the rbatis-codegen operator modules:
//! ops_add, ops_sub, ops_mul, ops_div, ops_rem, ops_shl, ops_shr,
//! ops_neg, ops_not, ops_xor, ops_bit_and, ops_bit_or, ops_str and the
//! macro bodies in ops_macros.rs.
//!
//! Every custom trait impl is called explicitly as a method so that the
//! `rbs::Value` operator matrix (owned / & / && receivers, `Value`-`Value`,
//! `Value`-primitive and primitive-`Value`) runs at runtime.

#[cfg(test)]
mod test {
    use rbatis_codegen::ops::{
        Add, BitAnd, BitOr, BitShl, BitShr, BitXor, Div, Mul, Neg, Not, Rem, Str, Sub,
    };
    use rbs::Value;

    // ------------------------------------------------------------------
    // Call-matrix helpers
    // ------------------------------------------------------------------

    /// Runs all nine `Value` receiver/operand reference combinations.
    macro_rules! check_value_matrix {
        ($method:ident, $lhs:expr, $rhs:expr, $expected:expr) => {{
            let l = $lhs;
            let r = $rhs;
            let e = $expected;
            assert_eq!(l.clone().$method(r.clone()), e.clone());
            assert_eq!(l.clone().$method(&r), e.clone());
            assert_eq!(l.clone().$method(&&r), e.clone());
            assert_eq!((&l).$method(r.clone()), e.clone());
            assert_eq!((&l).$method(&r), e.clone());
            assert_eq!((&l).$method(&&r), e.clone());
            assert_eq!((&&l).$method(r.clone()), e.clone());
            assert_eq!((&&l).$method(&r), e.clone());
            assert_eq!((&&l).$method(&&r), e.clone());
        }};
    }

    /// Runs the twelve `Value`/primitive combinations (six forward, six reverse).
    /// `$fwd` is the forward result and `$rev` the primitive-op-`Value` result.
    macro_rules! check_num12 {
        ($method:ident, $v:expr, $x:expr, $fwd:expr, $rev:expr) => {{
            let v: Value = $v;
            let x = $x;
            let f = $fwd;
            let r = $rev;
            // Value/&Value/&&Value receiver with primitive or &primitive operand
            assert_eq!(v.clone().$method(x), f);
            assert_eq!(v.clone().$method(&x), f);
            assert_eq!((&v).$method(x), f);
            assert_eq!((&v).$method(&x), f);
            assert_eq!((&&v).$method(x), f);
            assert_eq!((&&v).$method(&x), f);
            // primitive/&primitive receiver with Value/&Value/&&Value operand
            assert_eq!(x.$method(v.clone()), r);
            assert_eq!(x.$method(&v), r);
            assert_eq!(x.$method(&&v), r);
            assert_eq!((&x).$method(v.clone()), r);
            assert_eq!((&x).$method(&v), r);
            assert_eq!((&x).$method(&&v), r);
        }};
    }

    /// Runs the four impls of `impl_same_type_binary_op` / `impl_same_type_shift_op`.
    macro_rules! check_same4 {
        ($method:ident, $a:expr, $b:expr, $expected:expr) => {{
            let a = $a;
            let b = $b;
            let e = $expected;
            assert_eq!(a.$method(b), e);
            assert_eq!(a.$method(&b), e);
            assert_eq!((&a).$method(b), e);
            assert_eq!((&a).$method(&b), e);
        }};
    }

    /// Runs all four unary receiver impls (`Value`, `&Value`, `&&Value`, `&mut Value`).
    macro_rules! check_unary4 {
        ($method:ident, $v:expr, $expected:expr) => {{
            let v = $v;
            let e = $expected;
            assert_eq!(v.clone().$method(), e.clone());
            assert_eq!((&v).$method(), e.clone());
            assert_eq!((&&v).$method(), e.clone());
            let mut m = v.clone();
            assert_eq!((&mut m).$method(), e.clone());
        }};
    }

    /// Runs all twelve `impl_bool_scalar_binary_op` impls.
    macro_rules! check_bool_scalar12 {
        ($method:ident, $v:expr, $b:expr, $expected:expr) => {{
            let v: Value = $v;
            let b: bool = $b;
            let e: bool = $expected;
            assert_eq!(v.clone().$method(b), e);
            assert_eq!(v.clone().$method(&b), e);
            assert_eq!((&v).$method(b), e);
            assert_eq!((&v).$method(&b), e);
            assert_eq!((&&v).$method(b), e);
            assert_eq!((&&v).$method(&b), e);
            assert_eq!(b.$method(v.clone()), e);
            assert_eq!(b.$method(&v), e);
            assert_eq!(b.$method(&&v), e);
            assert_eq!((&b).$method(v.clone()), e);
            assert_eq!((&b).$method(&v), e);
            assert_eq!((&b).$method(&&v), e);
        }};
    }

    // ==================================================================
    // ops_add.rs
    // ==================================================================

    #[test]
    fn add_value_matrix_and_kinds() {
        // impl_value_binary_op!: nine reference combinations.
        check_value_matrix!(op_add, Value::I32(2), Value::I32(3), Value::I32(5));

        // String arm: string + string concatenates ...
        assert_eq!(
            Value::String("ab".into()).op_add(Value::String("cd".into())),
            Value::String("abcd".into())
        );
        // ... and a non-string rhs contributes an empty suffix.
        assert_eq!(
            Value::String("ab".into()).op_add(Value::I32(9)),
            Value::String("ab".into())
        );

        // I32: checked_add success / overflow widening to I64.
        assert_eq!(Value::I32(2).op_add(Value::I64(3)), Value::I32(5));
        assert_eq!(
            Value::I32(i32::MAX).op_add(Value::I32(1)),
            Value::I64(i32::MAX as i64 + 1)
        );
        // I64: saturating add.
        assert_eq!(Value::I64(2).op_add(Value::I64(3)), Value::I64(5));
        assert_eq!(
            Value::I64(i64::MAX).op_add(Value::I64(1)),
            Value::I64(i64::MAX)
        );
        // U32: checked_add success / overflow widening to U64.
        assert_eq!(Value::U32(2).op_add(Value::U32(3)), Value::U32(5));
        assert_eq!(
            Value::U32(u32::MAX).op_add(Value::U32(1)),
            Value::U64(u32::MAX as u64 + 1)
        );
        // U64: saturating add.
        assert_eq!(Value::U64(2).op_add(Value::U64(3)), Value::U64(5));
        assert_eq!(
            Value::U64(u64::MAX).op_add(Value::U64(1)),
            Value::U64(u64::MAX)
        );
        // F64 / F32 arms.
        assert_eq!(Value::F64(1.5).op_add(Value::I32(2)), Value::F64(3.5));
        assert_eq!(Value::F32(1.5).op_add(Value::I32(2)), Value::F32(3.5));
        // Ext arm delegates to the wrapped value.
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(3))).op_add(Value::I32(4)),
            Value::I32(7)
        );
        assert_eq!(
            Value::Ext("custom", Box::new(Value::String("x".into())))
                .op_add(Value::String("y".into())),
            Value::String("xy".into())
        );
        // Fallback arm: every non-numeric left value is Null.
        assert_eq!(Value::Null.op_add(Value::I32(1)), Value::Null);
        assert_eq!(Value::Bool(true).op_add(Value::I32(1)), Value::Null);
        assert_eq!(Value::Array(vec![]).op_add(Value::I32(1)), Value::Null);
        assert_eq!(rbs::value!({}).op_add(Value::I32(1)), Value::Null);
    }

    #[test]
    fn add_numeric_matrix() {
        // unsigned group -> u64
        check_num12!(op_add, Value::I32(2), 3u8, 5u64, 5u64);
        check_num12!(op_add, Value::I32(2), 3u16, 5u64, 5u64);
        check_num12!(op_add, Value::I32(2), 3u32, 5u64, 5u64);
        check_num12!(op_add, Value::I32(2), 3u64, 5u64, 5u64);
        // signed group -> i64
        check_num12!(op_add, Value::I32(2), 3i8, 5i64, 5i64);
        check_num12!(op_add, Value::I32(2), 3i16, 5i64, 5i64);
        check_num12!(op_add, Value::I32(2), 3i32, 5i64, 5i64);
        check_num12!(op_add, Value::I32(2), 3i64, 5i64, 5i64);
        check_num12!(op_add, Value::I32(2), 3isize, 5i64, 5i64);
        check_num12!(op_add, Value::I32(2), 3usize, 5i64, 5i64);
        // float group -> f64
        check_num12!(op_add, Value::I32(2), 3f32, 5.0f64, 5.0f64);
        check_num12!(op_add, Value::I32(2), 3f64, 5.0f64, 5.0f64);

        // Saturation / numeric helper bodies.
        assert_eq!(Value::U64(u64::MAX).op_add(1u64), u64::MAX);
        assert_eq!(1u64.op_add(Value::U64(u64::MAX)), u64::MAX);
        assert_eq!(Value::I64(i64::MAX).op_add(1i64), i64::MAX);
        assert_eq!(1i64.op_add(Value::I64(i64::MAX)), i64::MAX);
        assert_eq!(Value::F64(1.5).op_add(2.25f64), 3.75f64);
        assert_eq!(Value::F64(1.5).op_add(2.25f32), 3.75f64);
    }

    #[test]
    fn add_same_type_matrix() {
        check_same4!(op_add, 2u8, 3u8, 5u8);
        check_same4!(op_add, 2u16, 3u16, 5u16);
        check_same4!(op_add, 2u32, 3u32, 5u32);
        check_same4!(op_add, 2u64, 3u64, 5u64);
        check_same4!(op_add, 2i8, 3i8, 5i8);
        check_same4!(op_add, 2i16, 3i16, 5i16);
        check_same4!(op_add, 2i32, 3i32, 5i32);
        check_same4!(op_add, 2i64, 3i64, 5i64);
        check_same4!(op_add, 2isize, 3isize, 5isize);
        check_same4!(op_add, 2usize, 3usize, 5usize);
        check_same4!(op_add, 2f32, 3f32, 5f32);
        check_same4!(op_add, 2f64, 3f64, 5f64);
    }

    #[test]
    fn add_value_string_rhs_matrix() {
        // Add<&str/&&str/String/&String/&&String> for Value, &Value, &&Value.
        let v = Value::String("L".into());
        let rs: &str = "R";
        assert_eq!(v.clone().op_add(rs), Value::String("LR".into()));
        assert_eq!((&v).op_add(rs), Value::String("LR".into()));
        assert_eq!((&&v).op_add(rs), Value::String("LR".into()));

        let rrs: &&str = &"R";
        assert_eq!(v.clone().op_add(rrs), Value::String("LR".into()));
        assert_eq!((&v).op_add(rrs), Value::String("LR".into()));
        assert_eq!((&&v).op_add(rrs), Value::String("LR".into()));

        let s = String::from("R");
        assert_eq!(v.clone().op_add(s.clone()), Value::String("LR".into()));
        assert_eq!((&v).op_add(s.clone()), Value::String("LR".into()));
        assert_eq!((&&v).op_add(s.clone()), Value::String("LR".into()));

        let sr = &s;
        assert_eq!(v.clone().op_add(sr), Value::String("LR".into()));
        assert_eq!((&v).op_add(sr), Value::String("LR".into()));
        assert_eq!((&&v).op_add(sr), Value::String("LR".into()));

        let ssr = &sr;
        assert_eq!(v.clone().op_add(ssr), Value::String("LR".into()));
        assert_eq!((&v).op_add(ssr), Value::String("LR".into()));
        assert_eq!((&&v).op_add(ssr), Value::String("LR".into()));
    }

    #[test]
    fn add_string_lhs_value_rhs_matrix() {
        // Add<Value/&Value/&&Value> for &str, &&str, String, &String, &&String.
        let v = Value::String("R".into());

        assert_eq!("L".op_add(v.clone()), "LR");
        assert_eq!("L".op_add(&v), "LR");
        assert_eq!("L".op_add(&&v), "LR");

        let l: &&str = &"L";
        assert_eq!(l.op_add(v.clone()), "LR");
        assert_eq!(l.op_add(&v), "LR");
        assert_eq!(l.op_add(&&v), "LR");

        let s = String::from("L");
        assert_eq!(s.clone().op_add(v.clone()), "LR");
        assert_eq!(s.clone().op_add(&v), "LR");
        assert_eq!(s.clone().op_add(&&v), "LR");

        let sr = &s;
        assert_eq!(sr.op_add(v.clone()), "LR");
        assert_eq!(sr.op_add(&v), "LR");
        assert_eq!(sr.op_add(&&v), "LR");

        let ssr = &sr;
        assert_eq!(ssr.op_add(v.clone()), "LR");
        assert_eq!(ssr.op_add(&v), "LR");
        assert_eq!(ssr.op_add(&&v), "LR");
    }

    #[test]
    fn add_explicit_string_impls() {
        // String lhs.
        assert_eq!(String::from("L").op_add("R"), "LR");
        assert_eq!(String::from("L").op_add(&"R"), "LR");
        assert_eq!(String::from("L").op_add(String::from("R")), "LR");
        let rs = String::from("R");
        assert_eq!(String::from("L").op_add(&rs), "LR");
        assert_eq!(String::from("L").op_add(&&rs), "LR");
        // &str lhs.
        assert_eq!("L".op_add(String::from("R")), "LR");
        assert_eq!("L".op_add(&rs), "LR");
        assert_eq!("L".op_add(&&rs), "LR");
        assert_eq!("L".op_add("R"), "LR");
        assert_eq!("L".op_add(&"R"), "LR");
        // &&str lhs.
        let l: &&str = &"L";
        assert_eq!(l.op_add("R"), "LR");
        assert_eq!(l.op_add(&"R"), "LR");
    }

    // ==================================================================
    // ops_sub.rs
    // ==================================================================

    #[test]
    fn sub_value_matrix_and_kinds() {
        check_value_matrix!(op_sub, Value::I32(5), Value::I32(3), Value::I32(2));

        assert_eq!(Value::I32(2).op_sub(Value::I64(3)), Value::I32(-1));
        assert_eq!(
            Value::I32(i32::MIN).op_sub(Value::I32(1)),
            Value::I64(i32::MIN as i64 - 1)
        );
        assert_eq!(Value::I64(5).op_sub(Value::I64(3)), Value::I64(2));
        assert_eq!(
            Value::I64(i64::MIN).op_sub(Value::I64(1)),
            Value::I64(i64::MIN)
        );
        assert_eq!(Value::U32(5).op_sub(Value::U32(3)), Value::U32(2));
        assert_eq!(
            Value::U32(0).op_sub(Value::U32(1)),
            Value::I64(-1)
        );
        assert_eq!(Value::U64(5).op_sub(Value::U64(3)), Value::U64(2));
        assert_eq!(Value::U64(0).op_sub(Value::U64(1)), Value::U64(0));
        assert_eq!(Value::F32(5.5).op_sub(Value::I32(2)), Value::F32(3.5));
        assert_eq!(Value::F64(5.5).op_sub(Value::I32(2)), Value::F64(3.5));
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(5))).op_sub(Value::I32(3)),
            Value::I32(2)
        );
        assert_eq!(Value::Null.op_sub(Value::I32(1)), Value::Null);
        assert_eq!(
            Value::String("x".into()).op_sub(Value::I32(1)),
            Value::Null
        );
    }

    #[test]
    fn sub_numeric_matrix() {
        check_num12!(op_sub, Value::I32(5), 3u8, 2u64, 0u64);
        check_num12!(op_sub, Value::I32(5), 3u16, 2u64, 0u64);
        check_num12!(op_sub, Value::I32(5), 3u32, 2u64, 0u64);
        check_num12!(op_sub, Value::I32(5), 3u64, 2u64, 0u64);
        check_num12!(op_sub, Value::I32(5), 3i8, 2i64, -2i64);
        check_num12!(op_sub, Value::I32(5), 3i16, 2i64, -2i64);
        check_num12!(op_sub, Value::I32(5), 3i32, 2i64, -2i64);
        check_num12!(op_sub, Value::I32(5), 3i64, 2i64, -2i64);
        check_num12!(op_sub, Value::I32(5), 3isize, 2i64, -2i64);
        check_num12!(op_sub, Value::I32(5), 3usize, 2i64, -2i64);
        check_num12!(op_sub, Value::I32(5), 3f32, 2.0f64, -2.0f64);
        check_num12!(op_sub, Value::I32(5), 3f64, 2.0f64, -2.0f64);

        assert_eq!(Value::U64(0).op_sub(1u64), 0u64);
        assert_eq!(1u64.op_sub(Value::U64(u64::MAX)), 0u64);
        assert_eq!(Value::I64(i64::MIN).op_sub(1i64), i64::MIN);
        assert_eq!(1i64.op_sub(Value::I64(i64::MAX)), 1i64 - i64::MAX);
        assert_eq!(Value::F64(5.5).op_sub(2.25f64), 3.25f64);
        assert_eq!(Value::F64(5.5).op_sub(2.25f32), 3.25f64);
    }

    #[test]
    fn sub_same_type_matrix() {
        check_same4!(op_sub, 5u8, 3u8, 2u8);
        check_same4!(op_sub, 5u16, 3u16, 2u16);
        check_same4!(op_sub, 5u32, 3u32, 2u32);
        check_same4!(op_sub, 5u64, 3u64, 2u64);
        check_same4!(op_sub, 5i8, 3i8, 2i8);
        check_same4!(op_sub, 5i16, 3i16, 2i16);
        check_same4!(op_sub, 5i32, 3i32, 2i32);
        check_same4!(op_sub, 5i64, 3i64, 2i64);
        check_same4!(op_sub, 5isize, 3isize, 2isize);
        check_same4!(op_sub, 5usize, 3usize, 2usize);
        check_same4!(op_sub, 5f32, 3f32, 2f32);
        check_same4!(op_sub, 5f64, 3f64, 2f64);
    }

    // ==================================================================
    // ops_mul.rs
    // ==================================================================

    #[test]
    fn mul_value_matrix_and_kinds() {
        check_value_matrix!(op_mul, Value::I32(3), Value::I32(4), Value::I32(12));

        assert_eq!(Value::I32(3).op_mul(Value::I64(4)), Value::I32(12));
        assert_eq!(
            Value::I32(i32::MAX).op_mul(Value::I32(2)),
            Value::I64(i32::MAX as i64 * 2)
        );
        assert_eq!(Value::I64(3).op_mul(Value::I64(4)), Value::I64(12));
        assert_eq!(
            Value::I64(i64::MAX).op_mul(Value::I64(2)),
            Value::I64(i64::MAX)
        );
        assert_eq!(Value::U32(3).op_mul(Value::U32(4)), Value::U32(12));
        assert_eq!(
            Value::U32(u32::MAX).op_mul(Value::U32(2)),
            Value::U64(u32::MAX as u64 * 2)
        );
        assert_eq!(Value::U64(3).op_mul(Value::U64(4)), Value::U64(12));
        assert_eq!(
            Value::U64(u64::MAX).op_mul(Value::U64(2)),
            Value::U64(u64::MAX)
        );
        assert_eq!(Value::F32(1.5).op_mul(Value::I32(2)), Value::F32(3.0));
        assert_eq!(Value::F64(1.5).op_mul(Value::I32(2)), Value::F64(3.0));
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(3))).op_mul(Value::I32(4)),
            Value::I32(12)
        );
        assert_eq!(Value::Null.op_mul(Value::I32(2)), Value::Null);
    }

    #[test]
    fn mul_numeric_matrix() {
        check_num12!(op_mul, Value::I32(5), 3u8, 15u64, 15u64);
        check_num12!(op_mul, Value::I32(5), 3u16, 15u64, 15u64);
        check_num12!(op_mul, Value::I32(5), 3u32, 15u64, 15u64);
        check_num12!(op_mul, Value::I32(5), 3u64, 15u64, 15u64);
        check_num12!(op_mul, Value::I32(5), 3i8, 15i64, 15i64);
        check_num12!(op_mul, Value::I32(5), 3i16, 15i64, 15i64);
        check_num12!(op_mul, Value::I32(5), 3i32, 15i64, 15i64);
        check_num12!(op_mul, Value::I32(5), 3i64, 15i64, 15i64);
        check_num12!(op_mul, Value::I32(5), 3isize, 15i64, 15i64);
        check_num12!(op_mul, Value::I32(5), 3usize, 15i64, 15i64);
        check_num12!(op_mul, Value::I32(5), 3f32, 15.0f64, 15.0f64);
        check_num12!(op_mul, Value::I32(5), 3f64, 15.0f64, 15.0f64);

        assert_eq!(Value::U64(u64::MAX).op_mul(2u64), u64::MAX);
        assert_eq!(Value::I64(i64::MAX).op_mul(2i64), i64::MAX);
        assert_eq!(Value::F64(1.5).op_mul(2.25f64), 3.375f64);
        assert_eq!(Value::F64(1.5).op_mul(2.25f32), 3.375f64);
    }

    #[test]
    fn mul_same_type_matrix() {
        check_same4!(op_mul, 2u8, 3u8, 6u8);
        check_same4!(op_mul, 2u16, 3u16, 6u16);
        check_same4!(op_mul, 2u32, 3u32, 6u32);
        check_same4!(op_mul, 2u64, 3u64, 6u64);
        check_same4!(op_mul, 2i8, 3i8, 6i8);
        check_same4!(op_mul, 2i16, 3i16, 6i16);
        check_same4!(op_mul, 2i32, 3i32, 6i32);
        check_same4!(op_mul, 2i64, 3i64, 6i64);
        check_same4!(op_mul, 2isize, 3isize, 6isize);
        check_same4!(op_mul, 2usize, 3usize, 6usize);
        check_same4!(op_mul, 2f32, 3f32, 6f32);
        check_same4!(op_mul, 2f64, 3f64, 6f64);
    }

    // ==================================================================
    // ops_div.rs
    // ==================================================================

    #[test]
    fn div_value_matrix_and_kinds() {
        check_value_matrix!(op_div, Value::I32(12), Value::I32(4), Value::I32(3));

        // I32: integer fold, try_from overflow -> 0, float fallback, zero float.
        assert_eq!(Value::I32(12).op_div(Value::I32(0)), Value::I32(0));
        assert_eq!(
            Value::I32(i32::MIN).op_div(Value::I32(-1)),
            Value::I32(0)
        );
        assert_eq!(Value::I32(5).op_div(Value::F64(2.0)), Value::I32(2));
        assert_eq!(Value::I32(5).op_div(Value::F64(0.0)), Value::I32(0));
        // I64
        assert_eq!(Value::I64(12).op_div(Value::I64(4)), Value::I64(3));
        assert_eq!(Value::I64(12).op_div(Value::I64(0)), Value::I64(0));
        assert_eq!(
            Value::I64(i64::MIN).op_div(Value::I64(-1)),
            Value::I64(0)
        );
        assert_eq!(Value::I64(5).op_div(Value::F64(2.0)), Value::I64(2));
        assert_eq!(Value::I64(5).op_div(Value::F64(0.0)), Value::I64(0));
        // U32
        assert_eq!(Value::U32(12).op_div(Value::U32(4)), Value::U32(3));
        assert_eq!(Value::U32(12).op_div(Value::U32(0)), Value::U32(0));
        assert_eq!(Value::U32(5).op_div(Value::I32(-1)), Value::U32(0));
        assert_eq!(Value::U32(5).op_div(Value::F64(2.0)), Value::U32(2));
        assert_eq!(Value::U32(5).op_div(Value::F64(0.0)), Value::U32(0));
        // U64
        assert_eq!(Value::U64(12).op_div(Value::U64(4)), Value::U64(3));
        assert_eq!(Value::U64(12).op_div(Value::U64(0)), Value::U64(0));
        assert_eq!(Value::U64(5).op_div(Value::I32(-1)), Value::U64(0));
        assert_eq!(Value::U64(5).op_div(Value::F64(2.0)), Value::U64(2));
        assert_eq!(Value::U64(5).op_div(Value::F64(0.0)), Value::U64(0));
        // F32
        assert_eq!(Value::F32(5.0).op_div(Value::I32(2)), Value::F32(2.5));
        assert_eq!(Value::F32(5.0).op_div(Value::F64(0.0)), Value::F32(0.0));
        // F64
        assert_eq!(Value::F64(5.0).op_div(Value::I32(2)), Value::F64(2.5));
        assert_eq!(Value::F64(5.0).op_div(Value::F64(0.0)), Value::F64(0.0));
        // Ext delegates.
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(12))).op_div(Value::I32(3)),
            Value::I32(4)
        );
        // Fallback.
        assert_eq!(Value::Null.op_div(Value::I32(2)), Value::Null);
        assert_eq!(
            Value::Binary(vec![1]).op_div(Value::I32(2)),
            Value::Null
        );
    }

    #[test]
    fn div_numeric_matrix() {
        check_num12!(op_div, Value::I32(6), 3u8, 2u64, 0u64);
        check_num12!(op_div, Value::I32(6), 3u16, 2u64, 0u64);
        check_num12!(op_div, Value::I32(6), 3u32, 2u64, 0u64);
        check_num12!(op_div, Value::I32(6), 3u64, 2u64, 0u64);
        check_num12!(op_div, Value::I32(6), 3i8, 2i64, 0i64);
        check_num12!(op_div, Value::I32(6), 3i16, 2i64, 0i64);
        check_num12!(op_div, Value::I32(6), 3i32, 2i64, 0i64);
        check_num12!(op_div, Value::I32(6), 3i64, 2i64, 0i64);
        check_num12!(op_div, Value::I32(6), 3isize, 2i64, 0i64);
        check_num12!(op_div, Value::I32(6), 3usize, 2i64, 0i64);
        check_num12!(op_div, Value::I32(6), 3f32, 2.0f64, 0.5f64);
        check_num12!(op_div, Value::I32(6), 3f64, 2.0f64, 0.5f64);

        // Zero divisor guards, forward and reverse, for each helper.
        assert_eq!(Value::I32(5).op_div(0u64), 0u64);
        assert_eq!(5u64.op_div(Value::I32(0)), 0u64);
        assert_eq!(Value::I32(5).op_div(0i64), 0i64);
        assert_eq!(5i64.op_div(Value::I32(0)), 0i64);
        assert_eq!(Value::I32(5).op_div(0f64), 0.0f64);
        assert_eq!(5f64.op_div(Value::I32(0)), 0.0f64);
    }

    #[test]
    fn div_same_type_matrix() {
        check_same4!(op_div, 6u8, 3u8, 2u8);
        check_same4!(op_div, 6u16, 3u16, 2u16);
        check_same4!(op_div, 6u32, 3u32, 2u32);
        check_same4!(op_div, 6u64, 3u64, 2u64);
        check_same4!(op_div, 6i8, 3i8, 2i8);
        check_same4!(op_div, 6i16, 3i16, 2i16);
        check_same4!(op_div, 6i32, 3i32, 2i32);
        check_same4!(op_div, 6i64, 3i64, 2i64);
        check_same4!(op_div, 6isize, 3isize, 2isize);
        check_same4!(op_div, 6usize, 3usize, 2usize);
        check_same4!(op_div, 6f32, 3f32, 2f32);
        check_same4!(op_div, 6f64, 3f64, 2f64);
    }

    // ==================================================================
    // ops_rem.rs
    // ==================================================================

    #[test]
    fn rem_value_matrix_and_kinds() {
        check_value_matrix!(op_rem, Value::I32(13), Value::I32(5), Value::I32(3));

        assert_eq!(Value::I32(13).op_rem(Value::I32(0)), Value::I32(0));
        assert_eq!(Value::I32(5).op_rem(Value::F64(2.0)), Value::I32(1));
        assert_eq!(Value::I32(5).op_rem(Value::F64(0.0)), Value::I32(0));
        assert_eq!(Value::I64(13).op_rem(Value::I64(5)), Value::I64(3));
        assert_eq!(Value::I64(13).op_rem(Value::I64(0)), Value::I64(0));
        assert_eq!(Value::I64(5).op_rem(Value::F64(2.0)), Value::I64(1));
        assert_eq!(Value::I64(5).op_rem(Value::F64(0.0)), Value::I64(0));
        assert_eq!(Value::U32(13).op_rem(Value::U32(5)), Value::U32(3));
        assert_eq!(Value::U32(13).op_rem(Value::U32(0)), Value::U32(0));
        assert_eq!(Value::U32(5).op_rem(Value::F64(2.0)), Value::U32(1));
        assert_eq!(Value::U32(5).op_rem(Value::F64(0.0)), Value::U32(0));
        assert_eq!(Value::U64(13).op_rem(Value::U64(5)), Value::U64(3));
        assert_eq!(Value::U64(13).op_rem(Value::U64(0)), Value::U64(0));
        assert_eq!(Value::U64(5).op_rem(Value::F64(2.0)), Value::U64(1));
        assert_eq!(Value::U64(5).op_rem(Value::F64(0.0)), Value::U64(0));
        assert_eq!(Value::F32(5.5).op_rem(Value::I32(2)), Value::F32(1.5));
        assert_eq!(Value::F32(5.5).op_rem(Value::F64(0.0)), Value::F32(0.0));
        assert_eq!(Value::F64(5.5).op_rem(Value::I32(2)), Value::F64(1.5));
        assert_eq!(Value::F64(5.5).op_rem(Value::F64(0.0)), Value::F64(0.0));
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(13))).op_rem(Value::I32(5)),
            Value::I32(3)
        );
        assert_eq!(Value::Null.op_rem(Value::I32(2)), Value::Null);
    }

    #[test]
    fn rem_numeric_matrix() {
        check_num12!(op_rem, Value::I32(7), 3u8, 1u64, 3u64);
        check_num12!(op_rem, Value::I32(7), 3u16, 1u64, 3u64);
        check_num12!(op_rem, Value::I32(7), 3u32, 1u64, 3u64);
        check_num12!(op_rem, Value::I32(7), 3u64, 1u64, 3u64);
        check_num12!(op_rem, Value::I32(7), 3i8, 1i64, 3i64);
        check_num12!(op_rem, Value::I32(7), 3i16, 1i64, 3i64);
        check_num12!(op_rem, Value::I32(7), 3i32, 1i64, 3i64);
        check_num12!(op_rem, Value::I32(7), 3i64, 1i64, 3i64);
        check_num12!(op_rem, Value::I32(7), 3isize, 1i64, 3i64);
        check_num12!(op_rem, Value::I32(7), 3usize, 1i64, 3i64);
        check_num12!(op_rem, Value::I32(7), 3f32, 1.0f64, 3.0f64);
        check_num12!(op_rem, Value::I32(7), 3f64, 1.0f64, 3.0f64);

        assert_eq!(Value::I32(5).op_rem(0u64), 0u64);
        assert_eq!(5u64.op_rem(Value::I32(0)), 0u64);
        assert_eq!(Value::I32(5).op_rem(0i64), 0i64);
        assert_eq!(5i64.op_rem(Value::I32(0)), 0i64);
        assert_eq!(Value::I32(5).op_rem(0f64), 0.0f64);
        assert_eq!(5f64.op_rem(Value::I32(0)), 0.0f64);
    }

    #[test]
    fn rem_same_type_matrix() {
        check_same4!(op_rem, 7u8, 3u8, 1u8);
        check_same4!(op_rem, 7u16, 3u16, 1u16);
        check_same4!(op_rem, 7u32, 3u32, 1u32);
        check_same4!(op_rem, 7u64, 3u64, 1u64);
        check_same4!(op_rem, 7i8, 3i8, 1i8);
        check_same4!(op_rem, 7i16, 3i16, 1i16);
        check_same4!(op_rem, 7i32, 3i32, 1i32);
        check_same4!(op_rem, 7i64, 3i64, 1i64);
        check_same4!(op_rem, 7isize, 3isize, 1isize);
        check_same4!(op_rem, 7usize, 3usize, 1usize);
        check_same4!(op_rem, 5.5f32, 2.0f32, 1.5f32);
        check_same4!(op_rem, 5.5f64, 2.0f64, 1.5f64);
    }

    // ==================================================================
    // ops_shl.rs
    // ==================================================================

    #[test]
    fn shl_value_matrix_and_kinds() {
        check_value_matrix!(op_shl, Value::I32(1), Value::I32(2), Value::I32(4));

        assert_eq!(Value::I32(1).op_shl(Value::I32(2)), Value::I32(4));
        assert_eq!(Value::I32(1).op_shl(Value::I32(32)), Value::I32(0));
        assert_eq!(Value::I32(1).op_shl(Value::I32(-1)), Value::I32(0));
        assert_eq!(Value::I64(1).op_shl(Value::I32(2)), Value::I64(4));
        assert_eq!(Value::I64(1).op_shl(Value::I32(64)), Value::I64(0));
        assert_eq!(Value::I64(1).op_shl(Value::I32(-1)), Value::I64(0));
        assert_eq!(Value::U32(1).op_shl(Value::U32(2)), Value::U32(4));
        assert_eq!(Value::U32(1).op_shl(Value::U32(32)), Value::U32(0));
        assert_eq!(Value::U64(1).op_shl(Value::U64(2)), Value::U64(4));
        assert_eq!(Value::U64(1).op_shl(Value::U64(64)), Value::U64(0));
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(1))).op_shl(Value::I32(2)),
            Value::I32(4)
        );
        assert_eq!(Value::Null.op_shl(Value::I32(1)), Value::Null);
        assert_eq!(Value::F64(1.0).op_shl(Value::I32(1)), Value::Null);
    }

    #[test]
    fn shl_numeric_matrix() {
        check_num12!(op_shl, Value::I32(1), 3u8, 8u64, 6u64);
        check_num12!(op_shl, Value::I32(1), 3u16, 8u64, 6u64);
        check_num12!(op_shl, Value::I32(1), 3u32, 8u64, 6u64);
        check_num12!(op_shl, Value::I32(1), 3u64, 8u64, 6u64);
        check_num12!(op_shl, Value::I32(1), 3i8, 8i64, 6i64);
        check_num12!(op_shl, Value::I32(1), 3i16, 8i64, 6i64);
        check_num12!(op_shl, Value::I32(1), 3i32, 8i64, 6i64);
        check_num12!(op_shl, Value::I32(1), 3i64, 8i64, 6i64);
        check_num12!(op_shl, Value::I32(1), 3isize, 8i64, 6i64);
        check_num12!(op_shl, Value::I32(1), 3usize, 8i64, 6i64);

        // Forward guard: operand out of range.
        assert_eq!(Value::I32(1).op_shl(64u64), 0u64);
        assert_eq!(Value::I32(1).op_shl(-1i64), 0i64);
        assert_eq!(Value::I32(1).op_shl(64i64), 0i64);
        // Reverse guard: Value operand out of range.
        assert_eq!(1u64.op_shl(Value::U32(64)), 0u64);
        assert_eq!(1i64.op_shl(Value::I32(-1)), 0i64);
        assert_eq!(1i64.op_shl(Value::I32(64)), 0i64);
    }

    #[test]
    fn shl_same_type_matrix() {
        check_same4!(op_shl, 1u8, 3u8, 8u8);
        check_same4!(op_shl, 1u16, 3u16, 8u16);
        check_same4!(op_shl, 1u32, 3u32, 8u32);
        check_same4!(op_shl, 1u64, 3u64, 8u64);
        check_same4!(op_shl, 1i8, 3i8, 8i8);
        check_same4!(op_shl, 1i16, 3i16, 8i16);
        check_same4!(op_shl, 1i32, 3i32, 8i32);
        check_same4!(op_shl, 1i64, 3i64, 8i64);
        check_same4!(op_shl, 1isize, 3isize, 8isize);
        check_same4!(op_shl, 1usize, 3usize, 8usize);
        // checked_shl None -> unwrap_or_default() == 0.
        check_same4!(op_shl, 1u8, 8u8, 0u8);
        check_same4!(op_shl, 1i8, 8i8, 0i8);
        check_same4!(op_shl, 1u64, 64u64, 0u64);
        check_same4!(op_shl, 1i64, 64i64, 0i64);
    }

    // ==================================================================
    // ops_shr.rs
    // ==================================================================

    #[test]
    fn shr_value_matrix_and_kinds() {
        check_value_matrix!(op_shr, Value::I32(8), Value::I32(2), Value::I32(2));

        assert_eq!(Value::I32(8).op_shr(Value::I32(2)), Value::I32(2));
        assert_eq!(Value::I32(8).op_shr(Value::I32(32)), Value::I32(0));
        assert_eq!(Value::I32(8).op_shr(Value::I32(-1)), Value::I32(0));
        assert_eq!(Value::I64(8).op_shr(Value::I32(2)), Value::I64(2));
        assert_eq!(Value::I64(8).op_shr(Value::I32(64)), Value::I64(0));
        assert_eq!(Value::I64(8).op_shr(Value::I32(-1)), Value::I64(0));
        assert_eq!(Value::U32(8).op_shr(Value::U32(2)), Value::U32(2));
        assert_eq!(Value::U32(8).op_shr(Value::U32(32)), Value::U32(0));
        assert_eq!(Value::U64(8).op_shr(Value::U64(2)), Value::U64(2));
        assert_eq!(Value::U64(8).op_shr(Value::U64(64)), Value::U64(0));
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(8))).op_shr(Value::I32(2)),
            Value::I32(2)
        );
        assert_eq!(Value::Null.op_shr(Value::I32(1)), Value::Null);
        assert_eq!(Value::Bool(true).op_shr(Value::I32(1)), Value::Null);
    }

    #[test]
    fn shr_numeric_matrix() {
        check_num12!(op_shr, Value::I32(8), 2u8, 2u64, 0u64);
        check_num12!(op_shr, Value::I32(8), 2u16, 2u64, 0u64);
        check_num12!(op_shr, Value::I32(8), 2u32, 2u64, 0u64);
        check_num12!(op_shr, Value::I32(8), 2u64, 2u64, 0u64);
        check_num12!(op_shr, Value::I32(8), 2i8, 2i64, 0i64);
        check_num12!(op_shr, Value::I32(8), 2i16, 2i64, 0i64);
        check_num12!(op_shr, Value::I32(8), 2i32, 2i64, 0i64);
        check_num12!(op_shr, Value::I32(8), 2i64, 2i64, 0i64);
        check_num12!(op_shr, Value::I32(8), 2isize, 2i64, 0i64);
        check_num12!(op_shr, Value::I32(8), 2usize, 2i64, 0i64);

        assert_eq!(Value::I32(8).op_shr(64u64), 0u64);
        assert_eq!(Value::I32(8).op_shr(-1i64), 0i64);
        assert_eq!(Value::I32(8).op_shr(64i64), 0i64);
        assert_eq!(8u64.op_shr(Value::U32(64)), 0u64);
        assert_eq!(8i64.op_shr(Value::I32(-1)), 0i64);
        assert_eq!(8i64.op_shr(Value::I32(64)), 0i64);
    }

    #[test]
    fn shr_same_type_matrix() {
        check_same4!(op_shr, 8u8, 2u8, 2u8);
        check_same4!(op_shr, 8u16, 2u16, 2u16);
        check_same4!(op_shr, 8u32, 2u32, 2u32);
        check_same4!(op_shr, 8u64, 2u64, 2u64);
        check_same4!(op_shr, 8i8, 2i8, 2i8);
        check_same4!(op_shr, 8i16, 2i16, 2i16);
        check_same4!(op_shr, 8i32, 2i32, 2i32);
        check_same4!(op_shr, 8i64, 2i64, 2i64);
        check_same4!(op_shr, 8isize, 2isize, 2isize);
        check_same4!(op_shr, 8usize, 2usize, 2usize);
        check_same4!(op_shr, 8u8, 8u8, 0u8);
        check_same4!(op_shr, 8i8, 8i8, 0i8);
        check_same4!(op_shr, 8u64, 64u64, 0u64);
        check_same4!(op_shr, 8i64, 64i64, 0i64);
    }

    // ==================================================================
    // ops_neg.rs
    // ==================================================================

    #[test]
    fn neg_unary_matrix_and_kinds() {
        check_unary4!(neg, Value::I32(2), Value::I32(-2));

        // I32: checked_neg success and i32::MIN widening.
        assert_eq!(Value::I32(2).neg(), Value::I32(-2));
        assert_eq!(Value::I32(i32::MIN).neg(), Value::I64(2147483648));
        // I64: normal and i64::MIN saturation.
        assert_eq!(Value::I64(3).neg(), Value::I64(-3));
        assert_eq!(Value::I64(i64::MIN).neg(), Value::I64(i64::MAX));
        // F32 / F64.
        assert_eq!(Value::F32(1.5).neg(), Value::F32(-1.5));
        assert_eq!(Value::F64(2.5).neg(), Value::F64(-2.5));
        // Ext delegates.
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(3))).neg(),
            Value::I32(-3)
        );
        // Fallback.
        assert_eq!(Value::Null.neg(), Value::Null);
        assert_eq!(Value::Bool(true).neg(), Value::Null);
        assert_eq!(Value::String("x".into()).neg(), Value::Null);
        assert_eq!(Value::Binary(vec![1]).neg(), Value::Null);
    }

    // ==================================================================
    // ops_not.rs
    // ==================================================================

    #[test]
    fn not_unary_matrix_and_kinds() {
        check_unary4!(op_not, Value::I32(0), Value::I32(-1));

        assert_eq!(Value::I32(0).op_not(), Value::I32(-1));
        assert_eq!(Value::I64(0).op_not(), Value::I64(-1));
        assert_eq!(Value::U32(0).op_not(), Value::U32(u32::MAX));
        assert_eq!(Value::U64(0).op_not(), Value::U64(u64::MAX));
        assert_eq!(Value::Bool(true).op_not(), Value::Bool(false));
        assert_eq!(
            Value::Ext("custom", Box::new(Value::Bool(true))).op_not(),
            Value::Bool(false)
        );
        assert_eq!(Value::Null.op_not(), Value::Null);
        assert_eq!(Value::F64(1.0).op_not(), Value::Null);
        assert_eq!(Value::String("x".into()).op_not(), Value::Null);
    }

    // ==================================================================
    // ops_xor.rs
    // ==================================================================

    #[test]
    fn xor_value_matrix_and_kinds() {
        check_value_matrix!(op_bitxor, Value::I32(6), Value::I32(3), Value::I32(5));

        assert_eq!(
            Value::Bool(true).op_bitxor(Value::Bool(false)),
            Value::Bool(true)
        );
        assert_eq!(
            Value::Bool(true).op_bitxor(Value::I32(1)),
            Value::Bool(true)
        );
        assert_eq!(Value::I32(5).op_bitxor(Value::I32(3)), Value::I32(6));
        assert_eq!(Value::I64(5).op_bitxor(Value::I64(3)), Value::I64(6));
        assert_eq!(Value::U32(5).op_bitxor(Value::U32(3)), Value::U32(6));
        assert_eq!(Value::U64(5).op_bitxor(Value::U64(3)), Value::U64(6));
        assert_eq!(
            Value::Ext("custom", Box::new(Value::I32(5))).op_bitxor(Value::I32(3)),
            Value::I32(6)
        );
        assert_eq!(Value::Null.op_bitxor(Value::I32(3)), Value::Null);
        assert_eq!(
            Value::String("x".into()).op_bitxor(Value::I32(3)),
            Value::Null
        );
    }

    #[test]
    fn xor_bool_scalar_matrix() {
        check_bool_scalar12!(op_bitxor, Value::Bool(true), false, true);
        check_bool_scalar12!(op_bitxor, Value::Bool(true), true, false);
        // Non-bool Value is treated as false.
        check_bool_scalar12!(op_bitxor, Value::Null, true, true);
        check_bool_scalar12!(op_bitxor, Value::I32(1), true, true);
    }

    #[test]
    fn xor_numeric_matrix() {
        check_num12!(op_bitxor, Value::I32(6), 3u8, 5u64, 5u64);
        check_num12!(op_bitxor, Value::I32(6), 3u16, 5u64, 5u64);
        check_num12!(op_bitxor, Value::I32(6), 3u32, 5u64, 5u64);
        check_num12!(op_bitxor, Value::I32(6), 3u64, 5u64, 5u64);
        check_num12!(op_bitxor, Value::I32(6), 3i8, 5i64, 5i64);
        check_num12!(op_bitxor, Value::I32(6), 3i16, 5i64, 5i64);
        check_num12!(op_bitxor, Value::I32(6), 3i32, 5i64, 5i64);
        check_num12!(op_bitxor, Value::I32(6), 3i64, 5i64, 5i64);
        check_num12!(op_bitxor, Value::I32(6), 3isize, 5i64, 5i64);
        check_num12!(op_bitxor, Value::I32(6), 3usize, 5i64, 5i64);
        check_num12!(op_bitxor, Value::I32(6), 3f32, 5.0f64, 5.0f64);
        check_num12!(op_bitxor, Value::I32(6), 3f64, 5.0f64, 5.0f64);
    }

    #[test]
    fn xor_same_type_matrix() {
        check_same4!(op_bitxor, 6u8, 3u8, 5u8);
        check_same4!(op_bitxor, 6u16, 3u16, 5u16);
        check_same4!(op_bitxor, 6u32, 3u32, 5u32);
        check_same4!(op_bitxor, 6u64, 3u64, 5u64);
        check_same4!(op_bitxor, 6i8, 3i8, 5i8);
        check_same4!(op_bitxor, 6i16, 3i16, 5i16);
        check_same4!(op_bitxor, 6i32, 3i32, 5i32);
        check_same4!(op_bitxor, 6i64, 3i64, 5i64);
        check_same4!(op_bitxor, 6isize, 3isize, 5isize);
        check_same4!(op_bitxor, 6usize, 3usize, 5usize);
        check_same4!(op_bitxor, true, true, false);
        check_same4!(op_bitxor, true, false, true);
    }

    // ==================================================================
    // ops_bit_and.rs
    // ==================================================================

    #[test]
    fn bitand_value_matrix() {
        check_value_matrix!(
            op_bitand,
            Value::Bool(true),
            Value::Bool(true),
            true
        );
        check_value_matrix!(
            op_bitand,
            Value::Bool(true),
            Value::Bool(false),
            false
        );
        // `as_bool().unwrap_or(false)` for non-bool values.
        check_value_matrix!(op_bitand, Value::Null, Value::Bool(true), false);
        // Integers are not bools here, so the boolean fold yields false.
        check_value_matrix!(op_bitand, Value::I32(5), Value::I32(3), false);
    }

    #[test]
    fn bitand_bool_scalar_matrix() {
        check_bool_scalar12!(op_bitand, Value::Bool(true), true, true);
        check_bool_scalar12!(op_bitand, Value::Bool(true), false, false);
        check_bool_scalar12!(op_bitand, Value::Null, true, false);
    }

    #[test]
    fn bitand_numeric_matrix() {
        check_num12!(op_bitand, Value::I32(6), 3u8, 2u64, 2u64);
        check_num12!(op_bitand, Value::I32(6), 3u16, 2u64, 2u64);
        check_num12!(op_bitand, Value::I32(6), 3u32, 2u64, 2u64);
        check_num12!(op_bitand, Value::I32(6), 3u64, 2u64, 2u64);
        check_num12!(op_bitand, Value::I32(6), 3i8, 2i64, 2i64);
        check_num12!(op_bitand, Value::I32(6), 3i16, 2i64, 2i64);
        check_num12!(op_bitand, Value::I32(6), 3i32, 2i64, 2i64);
        check_num12!(op_bitand, Value::I32(6), 3i64, 2i64, 2i64);
        check_num12!(op_bitand, Value::I32(6), 3isize, 2i64, 2i64);
        check_num12!(op_bitand, Value::I32(6), 3usize, 2i64, 2i64);
    }

    #[test]
    fn bitand_same_type_matrix() {
        check_same4!(op_bitand, 6u8, 3u8, 2u8);
        check_same4!(op_bitand, 6u16, 3u16, 2u16);
        check_same4!(op_bitand, 6u32, 3u32, 2u32);
        check_same4!(op_bitand, 6u64, 3u64, 2u64);
        check_same4!(op_bitand, 6i8, 3i8, 2i8);
        check_same4!(op_bitand, 6i16, 3i16, 2i16);
        check_same4!(op_bitand, 6i32, 3i32, 2i32);
        check_same4!(op_bitand, 6i64, 3i64, 2i64);
        check_same4!(op_bitand, 6isize, 3isize, 2isize);
        check_same4!(op_bitand, 6usize, 3usize, 2usize);
        check_same4!(op_bitand, true, true, true);
        check_same4!(op_bitand, true, false, false);
    }

    // ==================================================================
    // ops_bit_or.rs
    // ==================================================================

    #[test]
    fn bitor_value_matrix() {
        check_value_matrix!(op_bitor, Value::Bool(false), Value::Bool(true), true);
        check_value_matrix!(op_bitor, Value::Bool(false), Value::Bool(false), false);
        check_value_matrix!(op_bitor, Value::Null, Value::Bool(true), true);
        // Integers fold through as_bool -> false | false.
        check_value_matrix!(op_bitor, Value::I32(5), Value::I32(3), false);
    }

    #[test]
    fn bitor_bool_scalar_matrix() {
        check_bool_scalar12!(op_bitor, Value::Bool(true), true, true);
        check_bool_scalar12!(op_bitor, Value::Bool(false), false, false);
        check_bool_scalar12!(op_bitor, Value::Null, true, true);
    }

    #[test]
    fn bitor_numeric_matrix() {
        check_num12!(op_bitor, Value::I32(6), 3u8, 7u64, 7u64);
        check_num12!(op_bitor, Value::I32(6), 3u16, 7u64, 7u64);
        check_num12!(op_bitor, Value::I32(6), 3u32, 7u64, 7u64);
        check_num12!(op_bitor, Value::I32(6), 3u64, 7u64, 7u64);
        check_num12!(op_bitor, Value::I32(6), 3i8, 7i64, 7i64);
        check_num12!(op_bitor, Value::I32(6), 3i16, 7i64, 7i64);
        check_num12!(op_bitor, Value::I32(6), 3i32, 7i64, 7i64);
        check_num12!(op_bitor, Value::I32(6), 3i64, 7i64, 7i64);
        check_num12!(op_bitor, Value::I32(6), 3isize, 7i64, 7i64);
        check_num12!(op_bitor, Value::I32(6), 3usize, 7i64, 7i64);
    }

    #[test]
    fn bitor_same_type_matrix() {
        check_same4!(op_bitor, 6u8, 3u8, 7u8);
        check_same4!(op_bitor, 6u16, 3u16, 7u16);
        check_same4!(op_bitor, 6u32, 3u32, 7u32);
        check_same4!(op_bitor, 6u64, 3u64, 7u64);
        check_same4!(op_bitor, 6i8, 3i8, 7i8);
        check_same4!(op_bitor, 6i16, 3i16, 7i16);
        check_same4!(op_bitor, 6i32, 3i32, 7i32);
        check_same4!(op_bitor, 6i64, 3i64, 7i64);
        check_same4!(op_bitor, 6isize, 3isize, 7isize);
        check_same4!(op_bitor, 6usize, 3usize, 7usize);
        check_same4!(op_bitor, false, true, true);
        check_same4!(op_bitor, false, false, false);
    }

    // ==================================================================
    // ops_str.rs
    // ==================================================================

    #[test]
    fn str_methods_matrix() {
        let v = Value::String("hello world".into());

        // Value receiver.
        assert!(v.clone().contains_str(Value::String("lo wo".into())));
        assert!(v.clone().starts_with(Value::String("hello".into())));
        assert!(v.clone().ends_with(Value::String("world".into())));
        assert!(!v.clone().contains_str(Value::String("xyz".into())));
        assert!(!v.clone().starts_with(Value::String("world".into())));
        assert!(!v.clone().ends_with(Value::String("hello".into())));

        // &Value receiver.
        assert!((&v).contains_str(Value::String("world".into())));
        assert!((&v).starts_with(Value::String("hello".into())));
        assert!((&v).ends_with(Value::String("world".into())));
        assert!(!(&v).contains_str(Value::String("xyz".into())));
        assert!(!(&v).starts_with(Value::String("xyz".into())));
        assert!(!(&v).ends_with(Value::String("xyz".into())));

        // &&Value receiver.
        assert!((&&v).contains_str(Value::String("world".into())));
        assert!((&&v).starts_with(Value::String("hello".into())));
        assert!((&&v).ends_with(Value::String("world".into())));
        assert!(!(&&v).contains_str(Value::String("xyz".into())));
        assert!(!(&&v).starts_with(Value::String("xyz".into())));
        assert!(!(&&v).ends_with(Value::String("xyz".into())));

        // Non-string values hit as_str().unwrap_or_default().
        assert!(!Value::Null.contains_str(Value::String("x".into())));
        assert!(!Value::Null.starts_with(Value::String("x".into())));
        assert!(!Value::Null.ends_with(Value::String("x".into())));
        assert!(!Value::I32(5).contains_str(Value::String("5".into())));
        assert!(!Value::Binary(vec![1]).starts_with(Value::String("1".into())));
        // Ext wraps a string and therefore is a string for these methods.
        assert!(
            Value::Ext("custom", Box::new(Value::String("abc".into()))).contains_str(Value::String("b".into()))
        );
    }
}
