
use std::cell::Cell;
use crate::operator::Operator;

pub struct Operators {
    operators: Vec<Operator>,
    next_index: Cell<usize>,
}

impl Operators {
    pub fn new(tokens: &Vec<&str>) -> Result<Self, i32> {
        let operators = tokens.iter()
            .map(|token| Operator::of(token))
            .filter(|operator| operator.is_some())
            .map(|operator| operator.unwrap())
            .collect();

        Ok(Operators {
            operators,
            next_index: Cell::new(0),
        })
    }

    pub fn len(&self) -> usize {
        self.operators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.next_index.get() == self.operators.len()
    }

    pub fn next(&self) -> Option<Operator> {
        let operator = self.operators[self.next_index.get()];
        self.next_index.set(self.next_index.get() + 1);
        return Some(operator);
    }
}