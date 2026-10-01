// src/pretty_print/pretty_expressions.rs

use super::pretty_print::{concat, group, line, nest, punctuate, space, text, Doc, ToDoc};
use crate::ir::ast::Expression;
use std::rc::Rc;

// --- Níveis de Precedência dos Operadores ---
// Define a ordem de operações para evitar o uso desnecessário de parênteses.
// Um operador com maior precedência é avaliado antes de um com menor precedência.
// Ex: `*` (PREC_MUL_DIV) tem maior precedência que `+` (PREC_ADD_SUB).
const PREC_NONE: usize = 0; // Contexto sem operador, como o topo de uma expressão.
const PREC_OR: usize = 1; // Precedência para o operador `or`.
const PREC_AND: usize = 2; // Precedência para o operador `and`.
const PREC_RELATIONAL: usize = 3; // Precedência para operadores como `==`, `>`, `<`.
const PREC_ADD_SUB: usize = 4; // Precedência para `+`, `-`.
const PREC_MUL_DIV: usize = 5; // Precedência para `*`, `/`.
const PREC_UNARY: usize = 6; // Precedência para operadores unários como `not`.
const PREC_CALL: usize = 7; // Precedência para chamadas de função e construtores.

// --- Implementação do Trait ---

/// Ponto de entrada para a conversão de uma `Expression` em `Doc`.
impl ToDoc for Expression {
    fn to_doc(&self) -> Rc<Doc> {
        // Inicia a conversão recursiva com a menor precedência possível.
        self.to_doc_inner(PREC_NONE)
    }
}

impl Expression {
    /// Função recursiva auxiliar que formata uma `Expression` em `Doc`,
    /// levando em conta a precedência do operador pai para decidir se
    /// parênteses são necessários.
    ///
    /// # Argumentos
    /// * `parent_precedence` - O nível de precedência da expressão externa (pai).
    fn to_doc_inner(&self, parent_precedence: usize) -> Rc<Doc> {
        // Closure que adiciona parênteses a um `Doc` se a precedência atual
        // for menor que a do operador pai. Ex: em `(a + b) * c`, `+` tem
        // menor precedência que `*`, então `a + b` precisa de parênteses.
        let maybe_paren = |current_precedence, doc| {
            if current_precedence < parent_precedence {
                concat(text("("), concat(doc, text(")")))
            } else {
                doc
            }
        };

        // Closure auxiliar para formatar operadores binários de forma consistente,
        // aplicando a lógica de parênteses.
        // Closure parametrizada para operadores binários.
        // Em operadores NÃO associativos (ex: -, /, comparações), o lado direito
        // recebe uma precedência artificialmente MAIOR para forçar parênteses
        // quando necessário e evitar reinterpretação incorreta.
        let binary_op =
            |op, current_prec, assoc: Associativity, lhs: &Expression, rhs: &Expression| {
                let rhs_prec = match assoc {
                    Associativity::Left => current_prec, // seguro para + e *
                    Associativity::NonAssociative => current_prec + 1, // força parênteses no RHS quando igual
                };
                let doc = concat(
                    lhs.to_doc_inner(current_prec),
                    concat(text(op), rhs.to_doc_inner(rhs_prec)),
                );
                maybe_paren(current_prec, doc)
            };

        #[derive(Copy, Clone)]
        enum Associativity {
            Left,
            NonAssociative,
        }

        match self {
            // Literais e variáveis são simples: apenas converte para texto.
            Expression::CTrue => text("True"),
            Expression::CFalse => text("False"),
            Expression::CInt(i) => text(i.to_string()),
            Expression::CReal(f) => text(f.to_string()),
            Expression::CString(s) => text(format!("\"{}\"", s)),
            Expression::CVoid => text("()"),
            Expression::Var(name) => text(name.clone()),

            // Formatação para todos os operadores binários usando a closure `binary_op`.
            Expression::Or(l, r) => binary_op(" or ", PREC_OR, Associativity::Left, l, r),
            Expression::And(l, r) => binary_op(" and ", PREC_AND, Associativity::Left, l, r),
            Expression::EQ(l, r) => {
                binary_op(" == ", PREC_RELATIONAL, Associativity::NonAssociative, l, r)
            }
            Expression::NEQ(l, r) => {
                binary_op(" != ", PREC_RELATIONAL, Associativity::NonAssociative, l, r)
            }
            Expression::GT(l, r) => {
                binary_op(" > ", PREC_RELATIONAL, Associativity::NonAssociative, l, r)
            }
            Expression::LT(l, r) => {
                binary_op(" < ", PREC_RELATIONAL, Associativity::NonAssociative, l, r)
            }
            Expression::GTE(l, r) => {
                binary_op(" >= ", PREC_RELATIONAL, Associativity::NonAssociative, l, r)
            }
            Expression::LTE(l, r) => {
                binary_op(" <= ", PREC_RELATIONAL, Associativity::NonAssociative, l, r)
            }
            Expression::Add(l, r) => binary_op(" + ", PREC_ADD_SUB, Associativity::Left, l, r),
            Expression::Sub(l, r) => {
                binary_op(" - ", PREC_ADD_SUB, Associativity::NonAssociative, l, r)
            }
            Expression::Mul(l, r) => binary_op(" * ", PREC_MUL_DIV, Associativity::Left, l, r),
            Expression::Div(l, r) => {
                binary_op(" / ", PREC_MUL_DIV, Associativity::NonAssociative, l, r)
            }

            // Formatação para operadores unários.
            Expression::Not(expr) => {
                let doc = concat(text("not "), expr.to_doc_inner(PREC_UNARY));
                maybe_paren(PREC_UNARY, doc)
            }
            Expression::Unwrap(expr) => concat(expr.to_doc_inner(PREC_CALL), text("!")),

            // Estruturas complexas que podem ter quebra de linha.
            Expression::FuncCall(name, args) => {
                let arg_docs: Vec<Rc<Doc>> =
                    args.iter().map(|a| a.to_doc_inner(PREC_NONE)).collect();
                let sep = concat(text(","), line());
                let joined = punctuate(sep, &arg_docs);
                concat(
                    text(name.clone()),
                    group(concat(text("("), concat(nest(4, joined), text(")")))),
                )
            }

            // ponytail: minimal rendering so the printer stays total; layout polish is future work.
            Expression::FieldAccess(obj, field) => {
                concat(obj.to_doc_inner(PREC_CALL), text(format!(".{}", field)))
            }
            Expression::MethodCall(obj, method, args) => {
                let arg_docs: Vec<Rc<Doc>> =
                    args.iter().map(|a| a.to_doc_inner(PREC_NONE)).collect();
                concat(
                    obj.to_doc_inner(PREC_CALL),
                    concat(
                        text(format!(".{}(", method)),
                        concat(punctuate(text(", "), &arg_docs), text(")")),
                    ),
                )
            }

            Expression::ListValue(elements) => {
                let elem_docs: Vec<Rc<Doc>> =
                    elements.iter().map(|e| e.to_doc_inner(PREC_NONE)).collect();
                let sep = concat(text(","), line());
                let joined = punctuate(sep, &elem_docs);
                group(concat(
                    text("["),
                    concat(nest(4, concat(line(), joined)), concat(line(), text("]"))),
                ))
            }

            Expression::Constructor(name, args) => {
                if args.is_empty() {
                    return text(name.clone());
                }
                let arg_docs: Vec<Rc<Doc>> =
                    args.iter().map(|a| a.to_doc_inner(PREC_NONE)).collect();
                let spaced = punctuate(space(), &arg_docs);
                let doc = concat(text(name.clone()), concat(space(), spaced));
                maybe_paren(PREC_CALL, doc)
            }

            // Tipos de tratamento de erro (Maybe/Result).
            Expression::COk(expr) => concat(text("Ok("), concat(expr.to_doc(), text(")"))),
            Expression::CErr(expr) => concat(text("Err("), concat(expr.to_doc(), text(")"))),
            Expression::CJust(expr) => concat(text("Just("), concat(expr.to_doc(), text(")"))),
            Expression::CNothing => text("Nothing"),

            // Predicados / inspeções
            Expression::IsError(expr) => concat(
                text("is_error("),
                concat(expr.to_doc_inner(PREC_NONE), text(")")),
            ),
            Expression::IsNothing(expr) => concat(
                text("is_nothing("),
                concat(expr.to_doc_inner(PREC_NONE), text(")")),
            ),

            // Propagação (usamos sintaxe pseudo-operador ?)
            Expression::Propagate(expr) => concat(expr.to_doc_inner(PREC_CALL), text("?")),

            // Lambda expressions
            Expression::Lambda(_func) => text("<lambda>"),

            // Tuple formatting: (a, b, c). Para 1 elemento usar (x,) se desejado (aqui seguimos Python style)
            Expression::Tuple(elements) => {
                if elements.is_empty() {
                    return text("()");
                }
                let elem_docs: Vec<Rc<Doc>> =
                    elements.iter().map(|e| e.to_doc_inner(PREC_NONE)).collect();
                let sep = concat(text(","), line());
                let joined = punctuate(sep, &elem_docs);
                group(concat(
                    text("("),
                    concat(nest(4, concat(line(), joined)), concat(line(), text(")"))),
                ))
            }
        }
    }
}

// (função join antiga removida – usar helpers punctuate / join_balanced)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::ast::Expression;
    use crate::pretty_print::pretty;

    #[test]
    fn test_precedence_add_mul() {
        let expr = Expression::Mul(
            Box::new(Expression::Add(
                Box::new(Expression::Var("a".to_string())),
                Box::new(Expression::Var("b".to_string())),
            )),
            Box::new(Expression::Var("c".to_string())),
        );
        assert_eq!(pretty(80, &expr.to_doc()), "(a + b) * c");
    }

    #[test]
    fn test_precedence_relational_and() {
        let expr = Expression::And(
            Box::new(Expression::GT(
                Box::new(Expression::Var("a".to_string())),
                Box::new(Expression::Var("b".to_string())),
            )),
            Box::new(Expression::LT(
                Box::new(Expression::Var("c".to_string())),
                Box::new(Expression::Var("d".to_string())),
            )),
        );
        assert_eq!(pretty(80, &expr.to_doc()), "a > b and c < d");
    }

    #[test]
    fn test_list_layout() {
        let list = Expression::ListValue(vec![
            Expression::CString("item longo".to_string()),
            Expression::CString("outro item longo".to_string()),
        ]);

        let doc = list.to_doc();

        // Com espaço suficiente, a lista é formatada em uma única linha.
        assert_eq!(
            pretty(100, &doc),
            "[ \"item longo\", \"outro item longo\" ]"
        );

        // Com espaço limitado, a lista quebra a linha e indenta os elementos.
        let narrow_expected = "[\n    \"item longo\",\n    \"outro item longo\"\n]";
        assert_eq!(pretty(20, &doc), narrow_expected);
    }

    #[test]
    fn test_precedence_sub_nested() {
        // a - (b - c) deve manter parênteses ao re-imprimir.
        let expr = Expression::Sub(
            Box::new(Expression::Var("a".into())),
            Box::new(Expression::Sub(
                Box::new(Expression::Var("b".into())),
                Box::new(Expression::Var("c".into())),
            )),
        );
        assert_eq!(pretty(80, &expr.to_doc()), "a - (b - c)");
    }

    #[test]
    fn test_precedence_div_nested() {
        // a / (b / c) deve manter parênteses.
        let expr = Expression::Div(
            Box::new(Expression::Var("a".into())),
            Box::new(Expression::Div(
                Box::new(Expression::Var("b".into())),
                Box::new(Expression::Var("c".into())),
            )),
        );
        assert_eq!(pretty(80, &expr.to_doc()), "a / (b / c)");
    }

    #[test]
    fn test_is_error_and_propagate_and_is_nothing() {
        let expr = Expression::IsError(Box::new(Expression::Propagate(Box::new(
            Expression::IsNothing(Box::new(Expression::Var("x".into()))),
        ))));
        assert_eq!(pretty(80, &expr.to_doc()), "is_error(is_nothing(x)?)");
    }
}
