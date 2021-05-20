use crate::operand::Operand;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLE,
    DIVIDE,
}

impl Operator {
    pub fn of(str: &str) -> Option<Operator> {
        match str {
            "+" => Some(Operator::PLUS),
            "-" => Some(Operator::MINUS),
            "*" => Some(Operator::MULTIPLE),
            "/" => Some(Operator::DIVIDE),
            _ => None,
        }
    }

    pub fn calculate(&self, operand0: Operand, operand1: Operand) -> i32 {
        let operand0 = operand0.num();
        let operand1 = operand1.num();
        match self {
            Operator::PLUS => operand0 + operand1,
            Operator::MINUS => operand0 - operand1,
            Operator::MULTIPLE => operand0 * operand1,
            Operator::DIVIDE => operand0 / operand1,
        }
    }

}