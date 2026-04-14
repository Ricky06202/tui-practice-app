use std::io::{Error, Stdout, Write, stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, raw::IntoRawMode};

pub struct UI {
    options: Vec<String>,
    selected: usize,
    title: String,
}

impl UI {
    pub fn new(title: &str, options: Vec<String>) -> Self {
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

    pub fn show(&mut self) -> Result<Option<usize>, Error> {
        let mut stdout = stdout().into_raw_mode()?;

        self.draw(&mut stdout)?;

        for key in stdin().keys() {
            match key? {
                Key::Char('q') => return Ok(None),
                Key::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                Key::Down => {
                    if self.selected < self.options.len() - 1 {
                        self.selected += 1;
                    }
                }
                Key::Char('\n') => return Ok(Some(self.selected)),
                _ => {}
            }
            self.draw(&mut stdout)?;
        }
        Ok(None)
    }
}
