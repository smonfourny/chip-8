pub struct OpCode {
    pub first: u8,
    pub second: u8,
}

pub trait HandleOp {
    fn handle_op(&self, op_code: &OpCode);
}