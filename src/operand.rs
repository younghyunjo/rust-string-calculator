use std::num::ParseIntError;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Operand {
    operand: i32,
}

impl Operand {
    pub fn new(operand: &str) -> Result<Self, ParseIntError> {
        let operand = operand.parse::<i32>()?;
        Operand::new_with_i32(operand)
    }

    pub fn new_with_i32(operand: i32) -> Result<Self, ParseIntError> {
        Ok(Operand {
            operand
        })
    }

    pub fn num(&self) -> i32 {
        self.operand
    }
}

#[cfg(test)]
mod test_operand {
    use crate::operand::Operand;

    #[test]
    fn given_string_when_new_then_created() {
        assert_eq!(Operand::new("1").unwrap(), Operand::new("1").unwrap());
    }

    #[test]
    fn given_two_digit_string_when_new_then_created() {
        assert_eq!(Operand::new("21").unwrap(), Operand::new_with_i32(21).unwrap());
    }

    #[test]
    fn given_wrong_string_when_new_then_error() {
        assert_eq!(Operand::new("+").is_err(), true);
    }
}
