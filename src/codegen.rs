use crate::{lexer::TokenKind, parser::Expr, Error};
use inkwell::{
    self, builder::Builder, context::Context, execution_engine::JitFunction, OptimizationLevel,
};

type SumFunc = unsafe extern "C" fn() -> f64;

pub fn jit_compile(ast: Expr<'_>) -> Result<f64, Error> {
    let context = Context::create();

    let module = context.create_module("main");
    let builder = context.create_builder();

    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::ExecutionEngine {
            message: e.to_string(),
        })?;

    let f64_type = context.f64_type();
    let fn_type = f64_type.fn_type(&[], false);
    let func_val = module.add_function("main", fn_type, None);

    let basic_block = context.append_basic_block(func_val, "entry");
    builder.position_at_end(basic_block);

    let return_val = compile_ast(ast, &context, &builder);
    builder.build_return(Some(&return_val));
    let main_func: JitFunction<SumFunc> = unsafe { execution_engine.get_function("main") }?;

    Ok(unsafe { main_func.call() })
}

fn compile_ast<'source, 'context, 'builder>(
    ast: Expr<'source>,
    context: &'context Context,
    builder: &'builder Builder<'context>,
) -> inkwell::values::FloatValue<'context> {
    use Expr::*;

    match ast {
        Number(n) => {
            let f64_type = context.f64_type();
            f64_type.const_float_from_string(n.lexeme)
        }
        BinOp { left, op, right } => {
            use TokenKind::*;

            let f64_type_left = compile_ast(*left, context, builder);
            let f64_type_right = compile_ast(*right, context, builder);

            match op.kind {
                Plus => builder.build_float_add(f64_type_left, f64_type_right, "sum"),
                Minus => builder.build_float_sub(f64_type_left, f64_type_right, "sub"),
                Slash => builder.build_float_div(f64_type_left, f64_type_right, "div"),
                Asterisk => builder.build_float_mul(f64_type_left, f64_type_right, "mul"),
                _ => unimplemented!(),
            }
        }
        Group(expr) => compile_ast(*expr, context, builder),
    }
}
