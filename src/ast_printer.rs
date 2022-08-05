use crate::expression::Expr;

pub fn ast_printer(expr: Expr) -> String {
    match expr {
        Expr::Literal { val } => format!("{}", val),
        Expr::Unary { op, expr } => format!("( {} {} )", op, ast_printer(*expr)),
        Expr::Binary { left, op, right } => {
            format!("( {} {} {} )", ast_printer(*left), op, ast_printer(*right))
        }
        Expr::Grouping { expr } => format!("( {} )", ast_printer(*expr)),
    }
}
