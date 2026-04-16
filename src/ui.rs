use std::io::{Error, Stdout, Write, stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, cursor, raw::IntoRawMode};

pub struct UI {
    options: Vec<String>,
    selected: usize,
    title: String,
}

pub fn clear(stdout: &mut Stdout) -> Result<(), Error> {
    write!(stdout, "{}", clear::All)?;
    stdout.flush()?;
    Ok(())
}

impl UI {
    pub fn new(title: &str, options: Vec<&str>) -> Self {
        UI {
            title: title.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
            selected: 0,
        }
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<(), Error> {
        clear(stdout)?;

        write!(stdout, "{}", cursor::Goto(1, 2))?;

        write!(stdout, "   {}\r\n", self.title)?;
        write!(stdout, "   {}\r\n\r\n", "=".repeat(self.title.len()))?;

        for (i, option) in self.options.iter().enumerate() {
            if i == self.selected {
                write!(stdout, " > {}\r\n", option)?;
            } else {
                write!(stdout, "   {}\r\n", option)?;
            }
        }

        write!(stdout, "\r\n\r\n")?;
        write!(
            stdout,
            "[ ↑↓ ] Navegar  |  [ Enter ] Seleccionar  |  [ Q ] Salir\r\n"
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

pub struct InputBox {
    prompt: String,
    value: String,
}

impl InputBox {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            value: String::new(),
        }
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<(), Error> {
        clear(stdout)?;

        write!(stdout, "{}", cursor::Goto(1, 2))?;

        write!(stdout, "   {}\r\n", self.prompt)?;
        write!(stdout, "   {}", "=".repeat(self.prompt.len()))?;

        write!(stdout, "\r\n\r\n")?;
        write!(stdout, "   {}", self.value)?;

        stdout.flush()?;
        Ok(())
    }

    pub fn show(&mut self) -> Result<String, Error> {
        let mut stdout = stdout().into_raw_mode()?;

        self.draw(&mut stdout)?;

        for key in stdin().keys() {
            match key? {
                Key::Char('\n') => return Ok(self.value.clone()),
                Key::Char(c) => self.value.push(c),
                Key::Backspace => {
                    self.value.pop();
                }
                _ => {}
            }
            self.draw(&mut stdout)?;
        }
        Ok(self.value.clone())
    }
}

pub struct MessageBox {
    title: String,
    message: String,
}

impl MessageBox {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
        }
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<(), Error> {
        clear(stdout)?;

        write!(stdout, "{}", cursor::Goto(1, 2))?;

        write!(stdout, "   {}\r\n", self.title)?;
        write!(stdout, "   {}", "=".repeat(self.title.len()))?;

        write!(stdout, "\r\n\r\n")?;
        for line in self.message.lines() {
            write!(stdout, "   {}\r\n", line)?;
        }

        write!(stdout, "\r\n\r\n")?;
        write!(stdout, "   [ Presione cualquier tecla para continuar ]")?;

        stdout.flush()?;
        Ok(())
    }

    pub fn show(&mut self) -> Result<(), Error> {
        let mut stdout = stdout().into_raw_mode()?;

        self.draw(&mut stdout)?;

        for key in stdin().keys() {
            match key? {
                Key::Char(_) => return Ok(()),
                _ => {}
            }
            self.draw(&mut stdout)?;
        }
        Ok(())
    }
}

pub struct SimulationBox {
    title: String,
    content: String,
    footer: String,
}

impl SimulationBox {
    pub fn new(title: &str, content: &str, footer: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            footer: footer.to_string(),
        }
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<(), Error> {
        clear(stdout)?;

        write!(stdout, "{}", cursor::Goto(1, 2))?;

        write!(stdout, "   {}\r\n", self.title)?;
        write!(stdout, "   {}", "=".repeat(self.title.len()))?;

        write!(stdout, "\r\n\r\n")?;
        for line in self.content.lines() {
            write!(stdout, "   {}\r\n", line)?;
        }

        write!(stdout, "\r\n\r\n")?;
        write!(stdout, "   {}", self.footer)?;

        stdout.flush()?;
        Ok(())
    }

    pub fn show(&mut self) -> Result<Key, Error> {
        let mut stdout = stdout().into_raw_mode()?;

        self.draw(&mut stdout)?;

        for key in stdin().keys() {
            return Ok(key?);
        }
        Err(Error::new(std::io::ErrorKind::Other, "Error al leer tecla"))
    }
}
