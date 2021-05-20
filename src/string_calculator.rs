use crate::input::Input;
use crate::operand::Operand;

pub fn string_calculator(term: &str) -> i32 {
    let input = Input::new(term);


    let mut operand0 = input.operand().unwrap();


    while !input.is_empty() {
        let operand1 = input.operand();
        let operator = input.operator();

        if operand1.is_none() || operator.is_none() {
            return operand0.num();
        }

        operand0 = Operand::new_with_i32(operator.unwrap().calculate(operand0, operand1.unwrap())).unwrap();
    }

    operand0.num()
}

#[cfg(test)]
mod tests_string_calculator {
    use crate::string_calculator::string_calculator;

    #[test]
    fn given_two_operands_when_add_then_added() {
        let ret = string_calculator("1 + 2");
        assert_eq!(ret, 3);
    }

    #[test]
    fn given_three_operands_when_add_then_added() {
        let ret = string_calculator("1 + 2 + 3");
        assert_eq!(ret, 6);
    }

    #[test]
    fn given_two_operand_when_minus_then_minus() {
        let ret = string_calculator("1 - 2");
        assert_eq!(ret, -1);
    }
}
