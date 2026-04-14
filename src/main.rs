mod game_of_life;
mod magic_square;
mod parenthesis;
mod pass_gen;
mod ui;
mod unit_convert;

use ui::UI;

fn main() {
    let options = vec![
        "Magic Square",
        "Parenthesis",
        "Game of Life",
        "Unit Converter",
        "Password Generator",
    ];
    let mut ui = UI::new("Main Menu", options);
    match ui.show() {
        Ok(Some(index)) => println!("Selected: {}", index),
        Ok(None) => println!("No selection"),
        Err(e) => println!("Error: {}", e),
    }
}
