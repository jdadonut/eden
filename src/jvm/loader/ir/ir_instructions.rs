use std::marker::PhantomData;
use super::ir_value::IrValue;


pub enum Instruction<T> {
    Arithmetic {
        l: T,
        r: T,
        op: ArithmeticOp,
    },

    NewSizedArray {
        size: usize,
        _ty: PhantomData<T>
    },
    NewDynamicArray {
        _ty: PhantomData<T>
    },




    Invoke {
        method: String,
        num_args: usize, // Number of arguments to pop off the stack
    },
    Focus {
        value: IrValue,
    }




    
}
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem
}
pub enum ArithmeticOpType {
    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,

    F32,
    F64
}