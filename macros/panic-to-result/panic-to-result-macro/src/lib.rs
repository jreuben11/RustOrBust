use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{token::Semi, Expr, ItemFn, ReturnType, Stmt, StmtMacro, Visibility};

fn signature_output_as_result(ast: &ItemFn) -> ReturnType {
    let output = match ast.sig.output {
        ReturnType::Default => {
            quote! {
              -> Result<(), String>
            }
        }
        ReturnType::Type(_, ref ty) => {
            if ty.to_token_stream().to_string().contains("Result") {
                // unimplemented!("cannot use macro on a function with Result as return type!");

                // return Err(
                //     syn::Error::new(
                //       ast.sig.span(),
                //       format!("this macro can only be applied to a function that does not return a Result. Signature: {}", quote!(#ty))
                //     )
                //   );

                emit_error!(ty,
                    format!(
                        "this macro can only be applied to a function that does not yet return a Result. Signature: {}",
                        quote!(#ty)
                    )
                );
                ast.sig.output.to_token_stream()
            } else {
                quote! {
                    -> Result<#ty, String>
                }
            }
        }
    };
    syn::parse2(output).unwrap()
}

fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
    let last_unwrapped = last_statement.unwrap();
    let last_modified = quote! {
      Ok(#last_unwrapped)
    };
    Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

fn extract_panic_content(expr_macro: &StmtMacro) -> Option<proc_macro2::TokenStream> {
    let does_panic = expr_macro
        .mac
        .path
        .segments
        .iter()
        .any(|v| v.ident.to_string().eq("panic"));
    if does_panic {
        Some(expr_macro.mac.tokens.clone())
    } else {
        None
    }
}

fn handle_expression_old(expression: Expr, token: Option<Semi>) -> Result<Stmt, syn::Error> {
    match expression {
        Expr::If(mut ex_if) => {
            let new_statements: Result<Vec<Stmt>, syn::Error> = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) => {
                        let output = extract_panic_content(expr_macro);
                        if output.map(|v| v.is_empty()).unwrap_or(false) {
                            Err(syn::Error::new(
                                expr_macro.span(),
                                format!("please make sure every panic in your function has a message, check: {}", quote!(#s))
                            ))
                        } else {
                            Ok(extract_panic_content(expr_macro)
                                // code to change panic to result
                                .map(|t| quote! {
                                        return Err(#t.to_string());
                                    }
                                )
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s)
                            )
                        }
                    }
                    _ => Ok(s),
                })
                .collect();
            ex_if.then_branch.stmts = new_statements?;
            Ok(Stmt::Expr(Expr::If(ex_if), token))
        }
        _ => Ok(Stmt::Expr(expression, token)),
    }
}

fn handle_expression(expression: Expr, token: Option<Semi>) -> Stmt {
    match expression {
        Expr::If(mut ex_if) => {
            let new_statements: Vec<Stmt> = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) => {
                        let output = extract_panic_content(expr_macro);

                        if output.map(|v| v.is_empty()).unwrap_or(false) {
                            emit_error!(expr_macro, "panic needs a message!".to_string();
                                    help = "try to add a message: panic!(\"Example\".to_string())";
                                    note = "we will add the message to Result's Err"
                            );
                            s
                            // alternative (does not return statement):
                            // abort!(expr_macro, "panic needs a message!".to_string();
                            //         help = "try to add a message: panic!(\"Example\".to_string())";
                            //         note = "we will add the message to Result's Err"
                            // );
                        } else {
                            extract_panic_content(expr_macro)
                                .map(|t| {
                                    quote! {
                                        return Err(#t.to_string());
                                    }
                                })
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s)
                        }
                    }
                    _ => s,
                })
                .collect();
            ex_if.then_branch.stmts = new_statements;
            Stmt::Expr(Expr::If(ex_if), token)
        }
        _ => Stmt::Expr(expression, token),
    }
}

#[proc_macro_attribute]
pub fn make_public(_a: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = syn::parse(item).unwrap();
    let new_ast = ItemFn {
        vis: Visibility::Public(Default::default()),
        ..ast
    };
    new_ast.to_token_stream().into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn panic_to_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).unwrap();

    let new_statements = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            Stmt::Expr(e, t) => handle_expression(e, t),
            _ => s,
        })
        .collect();
    ast.block.stmts = new_statements;

    ast.sig.output = signature_output_as_result(&ast);

    let last_statement = ast.block.stmts.pop();
    ast.block
        .stmts
        .push(last_statement_as_result(last_statement));

    ast.to_token_stream().into()
}
