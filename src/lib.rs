mod input;
mod operand;
mod operators;
mod operands;
mod operator;

pub mod string_calculator {
    use crate::{input, operand};


    pub fn calculate(term: &str) -> i32 {
        let mut input = input::Input::new(term);


        let mut operand0 = input.operand().unwrap();


        while !input.is_empty() {
            let operand1 = input.operand();
            let operator = input.operator();

            if operand1.is_none() || operator.is_none() {
                return operand0.num();
            }

            operand0 = operand::Operand::new_with_i32(operator.unwrap().calculate(operand0, operand1.unwrap())).unwrap();
        }

        operand0.num()
    }
}

#[cfg(test)]
mod tests_string_calculator {
    use crate::string_calculator::calculate;

    #[test]
    fn given_two_operands_when_add_then_added() {
        let ret = calculate("1 + 2");
        assert_eq!(ret, 3);
    }

    #[test]
    fn given_three_operands_when_add_then_added() {
        let ret = calculate("1 + 2 + 3");
        assert_eq!(ret, 6);
    }

    #[test]
    fn given_two_operand_when_minus_then_minus() {
        let ret = calculate("1 - 2");
        assert_eq!(ret, -1);
    }
}
