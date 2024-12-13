use super::enums::*;
use crate::lexer::enums::*;
use inkwell::context::Context;
use inkwell::types::{AnyType, BasicType};
use inkwell::values::{AnyValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::FloatPredicate;
use std::collections::HashMap;

pub struct CodegenContext<'ctx> {
    pub context: &'ctx Context,
    pub builder: inkwell::builder::Builder<'ctx>,
    pub module: inkwell::module::Module<'ctx>,
    pub variable_table: HashMap<String, PointerValue<'ctx>>, // Variables by name
    pub function_table: HashMap<String, FunctionValue<'ctx>>, // Functions by name
}

impl<'ctx> CodegenContext<'ctx> {
    // Insert variable into the context
    pub fn insert_variable(&mut self, name: String, ptr: PointerValue<'ctx>) {
        self.variable_table.insert(name, ptr);
    }

    // Retrieve variable from the context
    pub fn get_variable(&self, name: &str) -> Option<PointerValue<'ctx>> {
        self.variable_table.get(name).cloned()
    }

    // Insert function into the context
    pub fn insert_function(&mut self, name: String, func: FunctionValue<'ctx>) {
        self.function_table.insert(name, func);
    }

    // Retrieve function from the context
    pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.function_table.get(name).cloned()
    }
}

pub trait Codegen<'ctx> {
    fn generate_ir(&self, context: &mut CodegenContext<'ctx>) -> Result<BasicValueEnum<'ctx>, CodegenError>;
}

#[derive(Debug)]
pub enum CodegenError {
    BuildError(String),
}

impl<'ctx> Codegen<'ctx> for Statement {
    fn generate_ir(&self, context: &mut CodegenContext<'ctx>) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match self {
            // ignore for now i fail
            // too tored
            Statement::Return(value) => {
                let val = return context.context;
                let val = context.builder.build_return(value).unwrap();
                match *value {
                    Some(value) => {
                        
                        Ok(val.as_any_value_enum().into())
                        
                    },
                    None => Err(CodegenError::BuildError("Fail".into()))
                }

            },
            _ => todo!("I forgo")
        } 
    }
}

impl<'ctx> Codegen<'ctx> for LiteralValue {
    fn generate_ir(&self, context: &mut CodegenContext<'ctx>) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match self {
            LiteralValue::Number(value) => {
                let float_type = context.context.f64_type();
                let float_value = float_type.const_float(*value);
                Ok(float_value.into())
            }
            LiteralValue::String(value) => {
                let string_value = context.context.const_string(value.as_bytes(), false);
                Ok(string_value.into())
            }
            LiteralValue::Boolean(value) => {
                let bool_type = context.context.bool_type();
                let bool_value = bool_type.const_int(*value as u64, false);
                Ok(bool_value.into())
            }
            LiteralValue::Null => {
                let void_ptr_type = context.context.ptr_type(inkwell::AddressSpace::default());
                let null_value = void_ptr_type.const_null();
                Ok(null_value.into())
            }
        }
    }
}

impl<'ctx> Codegen<'ctx> for Expression {
    fn generate_ir(&self, context: &mut CodegenContext<'ctx>) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match self {
            
            Expression::While(cond, block) => {
                // Create basic blocks for the loop.
                let parent_fn = context.builder.get_insert_block().unwrap().get_parent().unwrap();
                let cond_block = context.context.append_basic_block(parent_fn, "whilecond");
                let body_block = context.context.append_basic_block(parent_fn, "whilebody");
                let end_block = context.context.append_basic_block(parent_fn, "endwhile");

                // Jump to the condition block.
                context.builder.build_unconditional_branch(cond_block);

                // Generate condition.
                context.builder.position_at_end(cond_block);
                let cond_value = cond.generate_ir(context)?.into_int_value();
                context.builder.build_conditional_branch(cond_value, body_block, end_block);

                // Generate the body.
                context.builder.position_at_end(body_block);
                for stmt in block {
                    stmt.generate_ir(context)?;
                }
                // After body, jump back to the condition block.
                context.builder.build_unconditional_branch(cond_block);

                // Continue after the loop.
                context.builder.position_at_end(end_block);
                match context.builder.build_unreachable() {
                    Ok(v) => Ok(v.into()),
                    Err(e) => Err(CodegenError::BuildError("Coped and failed, while loop func".into()))
                }
            }

            // Identifier_name would be i in for i in range(100) {}
            Expression::For(identifier_name, cond, block) => {
                let parent_fn = context.builder.get_insert_block().unwrap().get_parent().unwrap();
                let start_block = context.context.append_basic_block(parent_fn, "forstart");
                let body_block = context.context.append_basic_block(parent_fn, "forbody");
                let end_block = context.context.append_basic_block(parent_fn, "endfor");

                // Generate the range (assume cond is a range expression).
                let range_val = cond.generate_ir(context)?;

                // Create a pointer for the loop variable.
                let loop_var_ptr = context.builder.build_alloca(range_val.get_type(), identifier_name).unwrap();

                // Set up the initial value of the loop variable.
                let start_value = range_val.get_first_use().unwrap().get_used_value();
                context.builder.build_store(loop_var_ptr, start_value.left().unwrap());

                // Jump to the start block.
                context.builder.build_unconditional_branch(start_block);
                todo!()
                // Fucking myself bad
                // context.builder.position_at_end(start_block);
                // let pointee_type = context.context.ptr_type(inkwell::AddressSpace::default());
                // let current_val = context.builder.build_load(pointee_type, loop_var_ptr, identifier_name).unwrap().into_int_value();
                // let end_value = range_val;
                // let cond = context.builder.build_int_compare(IntPredicate::ULT, current_val, end_value, "forcond");
                // context.builder.build_conditional_branch(cond, body_block, end_block);
                //
                // // Generate the body of the loop.
                // context.builder.position_at_end(body_block);
                // for stmt in block {
                //     stmt.generate_ir(context)?;
                // }
                // // Increment the loop variable.
                // let incremented = context.builder.build_int_add(current_val, range_val.get_step(), "increment");
                // context.builder.build_store(loop_var_ptr, incremented);
                //
                // // Jump back to the condition.
                // context.builder.build_unconditional_branch(start_block);
                //
                // // End block.
                // context.builder.position_at_end(end_block);
                // Ok(context.builder.build_unreachable().into())
            }
            Expression::Assignment(name, val) => {

                todo!()
            }
            Expression::If(cond, block) => {

                todo!()
            }
            Expression::Keyword(name) => {

                todo!()
            }
            Expression::Grouping(expr) => {

                todo!()
            }
            Expression::IfElse(cond_1, block, other_cond_statement, else_statement) => {
                todo!()

            }
            Expression::BitwiseOp(cond_1, op, cond_2) => {

                todo!()
            }
            Expression::LogicalOp(cond_1, op, cond_2) => {

                todo!()
            }
            Expression::LiteralValue(literal) => literal.generate_ir(context),
            Expression::Identifier(name) => {
                if let Some(ptr) = context.get_variable(name) {
                    // Determine the pointee type by using the pointer's type.
                    let pointee_type = ptr.get_type().as_basic_type_enum();
                    // Load the value of the variable from memory
                    let value = context.builder.build_load(pointee_type, ptr, name);

                    match value {
                        Ok(v) => Ok(v),
                        Err(_) => Err(CodegenError::BuildError("I suck, identifier fail".into()))
                    }
                    
                } else {
                    Err(CodegenError::BuildError(format!("Undefined variable: {}", name)))
                }
            }
            Expression::MathOp(lhs, op, rhs) => {
                let lhs_val = lhs.generate_ir(context)?.into_float_value();
                let rhs_val = rhs.generate_ir(context)?.into_float_value();

                let result = match op {
                    MathOperator::Add => context.builder.build_float_add(lhs_val, rhs_val, "tmpadd"),
                    MathOperator::Subtract => context.builder.build_float_sub(lhs_val, rhs_val, "tmpsub"),
                    MathOperator::Multiply => context.builder.build_float_mul(lhs_val, rhs_val, "tmpmul"),
                    MathOperator::Divide => context.builder.build_float_div(lhs_val, rhs_val, "tmpdiv"),
                    MathOperator::Modulus => context.builder.build_float_rem(lhs_val, rhs_val, "tmprem"),
                };

                match result {
                    Ok(v) => Ok(v.into()),
                    Err(e) => Err(CodegenError::BuildError("Fail".into()))
                }
            }
            Expression::ComparisonOp(lhs, op, rhs) => {
                let lhs_val = lhs.generate_ir(context)?.into_float_value();
                let rhs_val = rhs.generate_ir(context)?.into_float_value();

                let result = match op {
                    ComparisonOperator::Equals => {
                        context.builder.build_float_compare(FloatPredicate::OEQ, lhs_val, rhs_val, "cmpeq")
                    }
                    ComparisonOperator::NotEquals => {
                        context.builder.build_float_compare(FloatPredicate::ONE, lhs_val, rhs_val, "cmpne")
                    }
                    ComparisonOperator::GreaterThan => {
                        context.builder.build_float_compare(FloatPredicate::OGT, lhs_val, rhs_val, "cmpgt")
                    }
                    ComparisonOperator::LessThan => {
                        context.builder.build_float_compare(FloatPredicate::OLT, lhs_val, rhs_val, "cmplt")
                    }
                    ComparisonOperator::GreaterThanEq => {
                        context.builder.build_float_compare(FloatPredicate::OGE, lhs_val, rhs_val, "cmpge")
                    }
                    ComparisonOperator::LessThanEq => {
                        context.builder.build_float_compare(FloatPredicate::OLE, lhs_val, rhs_val, "cmple")
                    }
                };

                match result {
                    Ok(v) => Ok(v.into()),
                    Err(e) => Err(CodegenError::BuildError("Fail".into()))
                }
            }
            Expression::FunctionCall(name, args) => {
                if let Some(func) = context.get_function(name) {
                    let arg_values: Vec<BasicMetadataValueEnum> = args
                        .iter()
                        .map(|arg| arg.generate_ir(context))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|v| v.into())
                        .collect();

                    // Perform the function call
                    let call_site_value = context.builder.build_call(func, &arg_values, "calltmp").unwrap();

                    // Check if the function call has a return value
                    let return_value = match func.get_type().get_return_type() {
                        Some(_) => call_site_value.try_as_basic_value(),
                        None => {
                            return Err(CodegenError::BuildError(format!(
                                "Function '{}' does not return a value.",
                                name
                            )))
                        }
                    };
                    if let Some(v) = return_value.left() {
                        Ok(v)
                    } else {
                        Err(CodegenError::BuildError(format!(
                            "Function '{}' does not return a value.",
                            name
                        )))
                    }
                    
                } else {
                    Err(CodegenError::BuildError(format!("Undefined function: {}", name)))
                }
            }
        }
    }
}
