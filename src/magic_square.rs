pub struct MagicSquare {
    size: usize,
    grid: Vec<Vec<i32>>,
}

impl MagicSquare {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![vec![0; size]; size],
        }
    }

    pub fn generate(&mut self) -> Result<(), String> {
        if self.size % 2 == 0 {
            return Err("El tamaño debe ser impar para este método".to_string());
        }

        let mut i = 0;
        let mut j = self.size / 2;
        self.grid[i][j] = 1;

        for num in 2..=(self.size * self.size) {
            // Calculamos la siguiente posición (arriba a la derecha)
            let mut next_i = if i == 0 { self.size - 1 } else { i - 1 };
            let mut next_j = if j == self.size - 1 { 0 } else { j + 1 };

            // Si la celda ya está ocupada, bajamos una fila
            if self.grid[next_i][next_j] != 0 {
                next_i = (i + 1) % self.size;
                next_j = j;
            }

            i = next_i;
            j = next_j;
            self.grid[i][j] = num as i32;
        }
        Ok(())
    }

    pub fn format_grid(&self) -> String {
        let max_num = (self.size * self.size) as i32;
        let width = max_num.to_string().len();
        let mut output = String::new();
        for row in &self.grid {
            for &num in row {
                output.push_str(&format!("{:>width$} ", num, width = width));
            }
            output.push_str("\n"); // UI handles indentation line by line now
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_square_3x3() {
        let mut magic = MagicSquare::new(3);
        assert!(magic.generate().is_ok());

        let grid = magic.grid;
        // Suma mágica para 3x3 es 15
        let sum_row0: i32 = grid[0].iter().sum();
        assert_eq!(sum_row0, 15);

        // Verificar todas las filas
        for row in &grid {
            assert_eq!(row.iter().sum::<i32>(), 15);
        }
    }

    #[test]
    fn test_magic_square_even_size() {
        let mut magic = MagicSquare::new(4);
        assert!(magic.generate().is_err());
    }
}

pub fn run() {
    use crate::ui::{InputBox, MessageBox};

    let mut input_box = InputBox::new("Ingrese el tamaño del cuadrado (N impar)");
    if let Ok(input) = input_box.show() {
        if let Ok(n) = input.trim().parse::<usize>() {
            let mut magic = MagicSquare::new(n);
            match magic.generate() {
                Ok(_) => {
                    let grid_str = magic.format_grid();
                    let mut msg =
                        MessageBox::new(&format!("Cuadrado Mágico {}x{}", n, n), &grid_str);
                    let _ = msg.show();
                }
                Err(e) => {
                    let mut msg = MessageBox::new("Error", &e);
                    let _ = msg.show();
                }
            }
        } else {
            let mut msg = MessageBox::new("Error", "Por favor ingrese un número válido.");
            let _ = msg.show();
        }
    }
}
