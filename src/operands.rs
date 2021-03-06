use crate::operand::Operand;

pub struct Operands {
    operands: Vec<Operand>,
}

type OperandsIntError = u32;

impl Operands {
    pub fn new(operands: &Vec<&str>) -> Result<Self, OperandsIntError> {
        let operands: Vec<Operand> = operands.iter()
            .map(|o| Operand::new(o))
            .filter_map(|o| match o {
                Ok(_) => Some(o.unwrap()),
                Err(_) => None
            })
            .rev()
            .collect();

        Ok(Operands {
            operands,
        })
    }

    pub fn len(&self) -> usize {
        self.operands.len()
    }
}

impl Iterator for Operands {
    type Item = Operand;

    fn next(&mut self) -> Option<Self::Item> {
        self.operands.pop()
    }
}


#[cfg(test)]
mod test_operands {
    use crate::operands::Operands;
    use crate::operands::Operand;

    #[test]
    fn given_operands_when_new_then_struct_created() {
        let given_operands = vec!["1", "2", "3", "4"];
        let operands = Operands::new(&given_operands).unwrap();
        assert_eq!(operands.len(), given_operands.len());
    }

    #[test]
    fn given_operands_when_new_then_struct_created2() {
        let given_operands = vec!["1", "2", "3", "4", "+"];
        let operands = Operands::new(&given_operands).unwrap();
        assert_eq!(operands.len(), 4);
    }

    #[test]
    fn given_operands_when_next_then_next_operand() {
        let mut operands = Operands::new(&vec!["1", "2"]).unwrap();
        assert_eq!(operands.next().unwrap(), Operand::new_with_i32(1).unwrap());
        assert_eq!(operands.next().unwrap(), Operand::new_with_i32(2).unwrap());
    }
}