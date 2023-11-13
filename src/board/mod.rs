#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(PartialEq)]
pub enum GameFinaleState {
    Win(Player),
    Draw,
    StillGoing,
}

pub struct Board {
    x_data: u16,
    y_data: u16,
}

impl Board {
    pub fn new() -> Self {
        Self {
            x_data: 0,
            y_data: 0,
        }
    }

    /// Example:
    /// ```
    /// index(x=1, y=2)
    ///     == (2 * 3) + 1
    ///     == 7
    ///
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    pub fn index(&self, x: i32, y: i32) -> i32 {
        (y * 3) + x
    }

    pub fn set_cell(&mut self, x: i32, y: i32, p: Player) -> Result<(), &str> {
        match (x, y) {
            (0..=2, 0..=2) => (),
            _ => return Err("Out of bounds"),
        }

        let i = self.index(x, y);

        match p {
            Player::X => {
                self.x_data |= 1 << i;
                Ok(())
            }
            Player::O => {
                self.y_data |= 1 << i;
                Ok(())
            }
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<Player> {
        let i = self.index(x, y);

        if ((self.x_data >> i) & 0b01) == 1 {
            Some(Player::X)
        } else if ((self.y_data >> i) & 0b01) == 1 {
            Some(Player::O)
        } else {
            None
        }
    }

    pub fn get_available_cells(&self) -> Vec<(i32, i32)> {
        let mut available_cells = vec![];

        for y in 0..3 {
            for x in 0..3 {
                if self.get_cell(x, y).is_none() {
                    available_cells.push((x, y));
                }
            }
        }

        available_cells
    }

    // Simply checks all horizontal, all vertical, and both diagonals.
    pub fn check_winner(&self) -> GameFinaleState {
        let check_three = |(x1, y1), (x2, y2), (x3, y3)| {
            let one = self.get_cell(x1, y1);
            let two = self.get_cell(x2, y2);
            let three = self.get_cell(x3, y3);

            if one.is_some() && one == two && two == three {
                one // THE ONE AND ONLY!!! RAAAAAHHHH
            } else {
                None
            }
        };

        // Horizontal
        for y in 0..3 {
            if let Some(p) = check_three((0, y), (1, y), (2, y)) {
                return GameFinaleState::Win(p);
            }
        }

        // Vertical
        for x in 0..3 {
            if let Some(p) = check_three((x, 0), (x, 1), (x, 2)) {
                return GameFinaleState::Win(p);
            }
        }

        // Diag 1
        if let Some(p) = check_three((0, 0), (1, 1), (2, 2)) {
            return GameFinaleState::Win(p);
        }

        // Diag 2
        if let Some(p) = check_three((0, 2), (1, 1), (2, 0)) {
            return GameFinaleState::Win(p);
        }

        // If the board is full, it's a draw, else, game is still on!
        if self.x_data & self.y_data == u16::MAX {
            GameFinaleState::Draw
        } else {
            GameFinaleState::StillGoing
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Using ═║╔ ╗╚ ╝╠ ╣╦ ╩ ╬ unicode (look for Char Map on Windows):
        let mut s = String::from(
            "
            ╔═══╦═══╦═══╗
            ║ 0 ║ 1 ║ 2 ║
            ╠═══╬═══╬═══╣
            ║ 3 ║ 4 ║ 5 ║
            ╠═══╬═══╬═══╣
            ║ 6 ║ 7 ║ 8 ║
            ╚═══╩═══╩═══╝",
        );

        for y in 0..3 {
            for x in 0..3 {
                let text = match self.get_cell(x, y) {
                    Some(Player::X) => "x",
                    Some(Player::O) => "o",
                    None => " ",
                };

                let i = self.index(x, y);

                s = s.replace(format!("{i}").as_str(), text);
            }
        }

        write!(f, "{s}")
    }
}
