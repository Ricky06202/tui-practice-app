use crate::ui::{InputBox, MessageBox, UI};

struct PasswordGenerator {
    length: usize,
    include_numbers: bool,
    include_symbols: bool,
    include_uppercase: bool,
}

impl PasswordGenerator {
    pub fn new(
        length: usize,
        include_numbers: bool,
        include_symbols: bool,
        include_uppercase: bool,
    ) -> Self {
        Self {
            length,
            include_numbers,
            include_symbols,
            include_uppercase,
        }
    }

    pub fn generate(&self) -> String {
        let mut charset = "abcdefghijklmnopqrstuvwxyz".to_string();
        if self.include_numbers {
            charset.push_str("0123456789");
        }
        if self.include_symbols {
            charset.push_str(r#"!@#$%^&*()_+-=[]{};':",./<>?"#);
        }
        if self.include_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        let mut password = String::new();
        for _ in 0..self.length {
            let random_index = rand::random_range(0..charset.len());
            password.push(charset.chars().nth(random_index).unwrap());
        }
        password
    }
}

pub fn run() {
    let mut password_generator = PasswordGenerator::new(16, true, true, true);
    loop {
        let mut ui = UI::new(
            "Password Generator",
            vec![
                "Generate",
                &format!("Change Length {}", password_generator.length),
                &format!("Include Numbers {}", password_generator.include_numbers),
                &format!("Include Symbols {}", password_generator.include_symbols),
                &format!("Include Uppercase {}", password_generator.include_uppercase),
            ],
        );
        match ui.show() {
            Ok(Some(index)) => match index {
                0 => {
                    let password = password_generator.generate();
                    let mut ui_result = MessageBox::new("Password Generated", &password);
                    ui_result.show().unwrap();
                }
                1 => {
                    let mut ui_length = InputBox::new("New Length");
                    let length_str = ui_length.show().unwrap();
                    let length = length_str.parse::<usize>().unwrap();
                    password_generator.length = length;
                }
                2 => {
                    password_generator.include_numbers = !password_generator.include_numbers;
                }
                3 => {
                    password_generator.include_symbols = !password_generator.include_symbols;
                }
                4 => {
                    password_generator.include_uppercase = !password_generator.include_uppercase;
                }
                _ => unreachable!(),
            },
            Ok(None) => break,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
