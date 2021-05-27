use crate::operand::Operand;

use crate::operators::Operators;
use crate::operands::Operands;
use crate::operator::Operator;


pub struct Input {
    operands: Operands,
    operators: Operators,

}

impl Input {
    pub fn new(input: &str) -> Self {
        let splitted = split(input);
        let operands = Operands::new(&splitted).unwrap();
        let operators = Operators::new(&splitted).unwrap();


        if operands.len() == 0 || operators.len() == 0 {
            panic!("wrong input")
        }
        if operands.len() != operators.len() + 1 {
            panic!("wrong operands, operations counts")
        }

        Self {
            operands: operands,
            operators,
        }
    }

    pub fn operand(&mut self) -> Option<Operand> {
        self.operands.next()
    }

    pub fn operator(&self) -> Option<&Operator> {
        self.operators.next()
    }

    pub fn is_empty(&self) -> bool {
        self.operators.is_empty()
    }
}


fn split(input: &str) -> Vec<&str> {
    input.trim().split_whitespace().collect()
}


#[cfg(test)]
mod tests_input {
    use crate::input::Input;
    use crate::operand::Operand;
    use crate::operator::Operator;

    #[test]
    fn when_operand_then_first_operand() {
        let mut  i = Input::new("1 + 2");
        assert_eq!(i.operand().unwrap(), Operand::new_with_i32(1).unwrap());
    }

    #[test]
    fn when_operand_two_times_then_second_operand() {
        let mut i = Input::new("1 + 2");
        assert_eq!(i.operand().unwrap(), Operand::new_with_i32(1).unwrap());
        assert_eq!(i.operand().unwrap(), Operand::new_with_i32(2).unwrap());
    }

    #[test]
    fn when_operation_then_first_operation() {
        let  i = Input::new("1 + 2");
        assert_eq!(i.operator().unwrap(), &Operator::PLUS);
    }

    #[test]
    #[should_panic(expected = "wrong input")]
    fn given_no_white_space_input_when_new_then_panic() {
        Input::new("1+2+ 3");
    }

    #[test]
    #[should_panic(expected = "wrong operands, operations counts")]
    fn given_wrong_input_when_new_then_panic() {
        Input::new("1 + 2 +");
    }
}
