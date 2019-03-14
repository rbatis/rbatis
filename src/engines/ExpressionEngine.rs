use std::collections::HashMap;

/**
* 表达式引擎，T:lexer类型,R 返回类型
**/
pub trait ExpressionEngine<T, R> {
    fn Name() -> String;
    fn Lexer(lexerArg: String) -> (T, String);
    fn Eval(lexerResult: T, arg: HashMap<&str, &str>) -> (R, String);
    fn LexerAndEval(lexerArg: String, arg: HashMap<&str, &str>) -> (R, String);
}