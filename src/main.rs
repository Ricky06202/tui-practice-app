mod game_of_life;
mod magic_square;
mod parenthesis;
mod pass_gen;
mod ui;
mod unit_convert;

use ui::UI;

fn main() {
    loop {
        let options = vec![
            "Magic Square",
            "Parenthesis",
            "Game of Life",
            "Unit Converter",
            "Password Generator",
        ];
        let mut ui = UI::new("Main Menu", options);
        match ui.show() {
            Ok(Some(index)) => match index {
                0 => magic_square::run(),
                1 => parenthesis::run(),
                2 => game_of_life::run(),
                3 => unit_convert::run(),
                4 => pass_gen::run(),
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
