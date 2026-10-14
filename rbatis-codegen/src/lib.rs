//! Compile time dynamic SQL generator.
//!
//! This is the compiler behind `#[py_sql]`, `#[html_sql]` and `#[sql]` (provided by
//! the `rbatis-macro-driver` crate and re-exported as
//! [`rbatis::rbatis_codegen`](https://docs.rs/rbatis/latest/rbatis/)). A dynamic SQL
//! template is parsed here and turned into plain Rust source code, which is then
//! compiled by the `rustc` invocation that is already running. That is why a query
//! written in `py_sql`/`html_sql` runs as fast as hand written SQL: nothing is
//! interpreted at runtime.
//!
//! * [`codegen::func`] holds the code generation functions
//! * [`codegen::parser_pysql`] and [`codegen::parser_html`] parse the two languages
//! * [`codegen::syntax_tree_pysql`] and [`codegen::syntax_tree_html`] are the
//!   abstract syntax trees they produce
//! * [`ops`] contains the operators that are available inside a template
//!   (`+`, `==`, `.sql()`, `.csv()`, ...)
//! * [`IntoSql`] renders a value as SQL, used by `${arg}` interpolation
//!
//! Most users never depend on this crate directly; it is documented because it is
//! re-exported from rbatis.

/// rendering a value as SQL, used by `${arg}` interpolation
pub mod into_sql;
pub use into_sql::IntoSql;
/// parsers and code generators of the `py_sql` and `html_sql` languages
pub mod codegen;
/// error type of the code generator
pub mod error;
/// `bool` helpers available inside a template expression
pub mod from_bool;
/// operators (`+`, `==`, `.sql()`, ...) usable inside a template
pub mod ops;
/// `+` operator
pub mod ops_add;
/// `&` (bit and) operator
pub mod ops_bit_and;
/// `|` (bit or) operator
pub mod ops_bit_or;
/// comparison operators (`>`, `<`, `>=`, `<=`)
pub mod ops_cmp;
/// `/` operator
pub mod ops_div;
/// `==` operator
pub mod ops_eq;
/// `*` operator
pub mod ops_mul;
/// unary `-` operator
pub mod ops_neg;
/// `!` operator
pub mod ops_not;
/// `%` operator
pub mod ops_rem;
/// `<<` operator
pub mod ops_shl;
/// `>>` operator
pub mod ops_shr;
/// string helpers (`.sql()`, `.csv()`, ...)
pub mod ops_string;
/// `-` operator
pub mod ops_sub;
/// `^` operator
pub mod ops_xor;
pub use codegen::{rb_html, rb_py};
