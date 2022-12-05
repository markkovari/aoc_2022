pub trait Stack<T> {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(self) -> Option<T>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoStack {
    pub elements: Vec<String>,
}

impl CargoStack {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }
    #[allow(dead_code)]
    pub fn transfer_top(&mut self, to: &mut CargoStack) {
        let item_to_transfer = self.pop();
        if item_to_transfer.is_some() {
            to.push(item_to_transfer.unwrap());
        }
    }
}

impl Stack<String> for CargoStack {
    fn push(self: &mut CargoStack, element: String) {
        self.elements.push(element)
    }
    fn pop(self: &mut CargoStack) -> Option<String> {
        self.elements.pop()
    }
    fn peek(self: CargoStack) -> Option<String> {
        match self.elements.get(self.elements.len() - 1) {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
}

#[cfg(test)]
mod test_stacks {
    use super::*;

    #[test]
    fn test_stack_transfer() {
        let mut first: CargoStack = CargoStack {
            elements: vec!["Z".to_owned(), "N".to_owned()],
        };
        let mut second: CargoStack = CargoStack {
            elements: vec!["M".to_owned(), "C".to_owned(), "D".to_owned()],
        };
        first.transfer_top(&mut second);
        assert_eq!(first.elements.len(), 1);
        assert_eq!(second.elements.len(), 4);
        assert_eq!(first.elements[0], "Z");
        assert_eq!(
            second.elements,
            vec![
                "M".to_owned(),
                "C".to_owned(),
                "D".to_owned(),
                "N".to_owned(),
            ]
        );
    }
}
