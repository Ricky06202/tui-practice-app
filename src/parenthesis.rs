use crate::ui::{InputBox, MessageBox};

pub struct ParenthesisChecker;

impl ParenthesisChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_balanced(&self, input: &str) -> bool {
        let mut stack = Vec::new();

        for c in input.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => continue,
            }
        }
        stack.is_empty()
    }
}

pub fn run() {
    let checker = ParenthesisChecker::new();
    let mut ui = InputBox::new("Ingrese una expresión con paréntesis () [] {}");
    let input = ui.show().unwrap();

    let result = checker.is_balanced(&input);
    let mut ui_result = MessageBox::new(
        "Resultado de la expresión, evaluando: () [] {}",
        &format!(
            "{}",
            if result {
                "Balanceado, no faltan paréntesis"
            } else {
                "No balanceado, falta cerrar paréntesis"
            }
        ),
    );
    ui_result.show().unwrap();
}
