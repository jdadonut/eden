#[derive(Debug, Clone)]
pub enum MethodHandleKind {
    #[allow(dead_code, non_camel_case_types)]
    __unused_ord_0,
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}
impl MethodHandleKind {
    pub fn to_ordinal(&self) -> u8 {
        match self {
            MethodHandleKind::__unused_ord_0 => 0,
            MethodHandleKind::GetField => 1,
            MethodHandleKind::GetStatic => 2,
            MethodHandleKind::PutField => 3,
            MethodHandleKind::PutStatic => 4,
            MethodHandleKind::InvokeVirtual => 5,
            MethodHandleKind::InvokeStatic => 6,
            MethodHandleKind::InvokeSpecial => 7,
            MethodHandleKind::NewInvokeSpecial => 8,
            MethodHandleKind::InvokeInterface => 9,
        }
    }
    pub fn from_ordinal(ordinal: u8) -> Option<Self> {
        match ordinal {
            1 => Some(MethodHandleKind::GetField),
            2 => Some(MethodHandleKind::GetStatic),
            3 => Some(MethodHandleKind::PutField),
            4 => Some(MethodHandleKind::PutStatic),
            5 => Some(MethodHandleKind::InvokeVirtual),
            6 => Some(MethodHandleKind::InvokeStatic),
            7 => Some(MethodHandleKind::InvokeSpecial),
            8 => Some(MethodHandleKind::NewInvokeSpecial),
            9 => Some(MethodHandleKind::InvokeInterface),
            _ => None,
        }
    }
}