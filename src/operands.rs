use crate::operand::Operand;
use std::cell::Cell;

pub struct Operands {
    operands: Vec<Operand>,
    next_index: Cell<usize>,
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
            .collect();

        Ok(Operands {
            operands,
            next_index: Cell::new(0),
        })
    }

    pub fn len(&self) -> usize {
        self.operands.len()
    }

    pub fn next(&self) -> Option<Operand> {
        let operand = self.operands[self.next_index.get()];
        self.next_index.set(self.next_index.get() + 1);
        return Some(operand);
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
        let operands = Operands::new(&vec!["1", "2"]).unwrap();
        assert_eq!(operands.next().unwrap(), Operand::new_with_i32(1).unwrap());
        assert_eq!(operands.next().unwrap(), Operand::new_with_i32(2).unwrap());
    }
}