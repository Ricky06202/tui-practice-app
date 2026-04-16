pub struct Universe {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    pub fn randomize(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = rand::random_bool(0.3); // 30% de probabilidad de vida inicial
            }
        }
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        for row in &self.grid {
            for &cell in row {
                output.push(if cell { '●' } else { '·' }); // '●' para vida, '·' para vacío
                output.push(' ');
            }
            output.push_str("\n");
        }
        output
    }

    pub fn evolve(&mut self) {
        let mut next_grid = self.grid.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                let neighbors = self.count_neighbors(i, j);

                if self.grid[i][j] {
                    // Supervivencia: Una célula viva con 2 o 3 vecinos vive.
                    // Muere si tiene menos de 2 (soledad) o más de 3 (superpoblación).
                    if neighbors < 2 || neighbors > 3 {
                        next_grid[i][j] = false;
                    }
                } else {
                    // Nacimiento: Una célula muerta con exactamente 3 vecinos nace.
                    if neighbors == 3 {
                        next_grid[i][j] = true;
                    }
                }
            }
        }
        self.grid = next_grid;
    }

    fn count_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let r = row as isize + i;
                let c = col as isize + j;

                if r >= 0 && r < self.height as isize && c >= 0 && c < self.width as isize {
                    if self.grid[r as usize][c as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reproduction() {
        // Una célula muerta con exactamente 3 vecinos debe nacer
        let mut universe = Universe::new(3, 3);
        universe.grid[0][0] = true;
        universe.grid[0][1] = true;
        universe.grid[0][2] = true;
        universe.evolve();
        assert!(universe.grid[1][1]);
    }

    #[test]
    fn test_underpopulation() {
        // Una célula viva con menos de 2 vecinos muere
        let mut universe = Universe::new(3, 3);
        universe.grid[1][1] = true;
        universe.grid[0][0] = true;
        universe.evolve();
        assert!(!universe.grid[1][1]);
    }
}

pub fn run() {
    use crate::ui::SimulationBox;
    use termion::event::Key;

    let mut universe = Universe::new(20, 10);
    universe.randomize();

    loop {
        let grid_str = universe.render();
        let mut sim_box = SimulationBox::new(
            "Conway's Game of Life",
            &grid_str,
            "[ Espacio ] Siguiente Generación  |  [ Q ] Salir",
        );

        match sim_box.show() {
            Ok(Key::Char('q')) => break,
            Ok(Key::Char(' ')) => {
                universe.evolve();
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }
}
