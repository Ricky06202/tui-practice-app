use std::io::{Error, Stdout, Write};
use termion::clear;

struct UI {
    options: Vec<String>,
    selected: usize,
    title: String,
}

impl UI {
    fn new(title: &str, options: Vec<String>) -> Self {
        UI {
            title: title.to_string(),
            options,
            selected: 0,
        }
    }

    fn clear(&self, stdout: &mut Stdout) -> Result<(), Error> {
        write!(stdout, "{}", clear::All)?;
        stdout.flush()?;
        Ok(())
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<(), Error> {
        self.clear(stdout)?;

        write!(stdout, "{}\n", self.title)?;
        write!(stdout, "{}\n\n", "=".repeat(self.title.len()))?;

        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected {
                write!(stdout, " > {}\n", option)?;
            } else {
                write!(stdout, "   {}\n", option)?;
            }
        }

        write!(stdout, "\n\n")?;
        write!(
            stdout,
            "[ ↑↓ ] Navegar  |  [ Enter ] Seleccionar  |  [ Q ] Salir"
        )?;

        stdout.flush()?;
        Ok(())
    }
}
