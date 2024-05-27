use super::ShouldPanicCollector;
use crate::{
    attributes::{AttributeInfo, ErrorExt},
    cairo_expression::CairoExpression,
    types::ParseFromExpr,
};
use cairo_lang_macro::Diagnostic;
use cairo_lang_syntax::node::{ast::Expr, db::SyntaxGroup};

#[derive(Debug, Clone, Default)]
pub enum Expected {
    ShortString(String),
    ByteArray(String),
    Array(Vec<String>),
    #[default]
    Any,
}

impl CairoExpression for Expected {
    fn as_cairo_expression(&self) -> String {
        match self {
            Self::ShortString(string) => {
                format!("snforge_std::_config_types::Expected::ShortString('{string}')")
            }
            Self::ByteArray(string) => {
                format!(r#"snforge_std::_config_types::Expected::ByteArray("{string}")"#)
            }
            Self::Array(strings) => {
                let arr = strings.join(",");

                format!("snforge_std::_config_types::Expected::Array([{arr}])")
            }
            Self::Any => "snforge_std::_config_types::Expected::Any".to_string(),
        }
    }
}

impl ParseFromExpr<Expr> for Expected {
    fn parse_from_expr<T: AttributeInfo>(
        db: &dyn SyntaxGroup,
        expr: &Expr,
        arg_name: &str,
    ) -> Result<Self, Diagnostic> {
        match expr {
            Expr::ShortString(string) => {
                let string = string.string_value(db).unwrap();

                Ok(Self::ShortString(string))
            }
            Expr::String(string) => {
                let string = string.string_value(db).unwrap();

                Ok(Self::ByteArray(string))
            }
            Expr::Tuple(expressions) => {
                let elements = expressions
                    .expressions(db)
                    .elements(db)
                    .into_iter()
                    .map(|expression| -> Result<String, Diagnostic> {
                        match expression {
                            Expr::ShortString(string) => {
                                let string = string.string_value(db).unwrap();

                                Ok(string)
                            }
                            Expr::Literal(string) => {
                                let string = string.numeric_value(db).unwrap();

                                Ok(format!("0x{}", string.to_str_radix(16)))
                            }
                            _ => Err(ShouldPanicCollector::error(format!(
                                "<{arg_name}> argument must be in form: {}",
                                ShouldPanicCollector::ARGS_FORM
                            )))?,
                        }
                    })
                    .collect::<Result<Vec<String>, Diagnostic>>()?;

                Ok(Self::Array(elements))
            }
            _ => Err(ShouldPanicCollector::error(format!(
                "<{arg_name}> argument must be in form: {}",
                ShouldPanicCollector::ARGS_FORM
            ))),
        }
    }
}
