use std::fmt::Display;
use rand::Rng;

#[derive(Clone, Copy)]
/// contains the information pertaining to a cell
pub struct Cell {
    is_mine: bool,
    /// number of mines surrounding this [Cell]
    local_mines: usize,
    /// represents if the user has picked to reveal contents of [Cell]
    is_revealed: bool,
    /// represents if the user has flagged the [Cell]
    is_flagged: bool,
}
impl Display for Cell {
    /// display as mine as "*". If empty, displays the number of mines in cells around it
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_revealed {
            if self.is_mine {
                write!(f, "*")?;
            } else {
                write!(f, "{}", self.local_mines)?;
            }
        } else if self.is_flagged {
            write!(f, "⚑")?;
        } else {
            write!(f, "#")?;
        }
        return Ok(());
    }
}
impl Cell {
    /// a clear cell used for a default value
    pub const CLEAR: Self = Self {
        local_mines: 0,
        is_mine: false,
        is_revealed: false,
        is_flagged: false,
    };
    /// used to generate a random cell
    pub fn random(is_mine_percentage: f64) -> Self {
        let mut rng = rand::thread_rng(); // thread specific random number generator
        return Self {
            is_mine: rng.gen_bool(is_mine_percentage),
            local_mines: 0,
            is_revealed: false,
            is_flagged: false,
        };
    }
    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }
    pub fn flag(&mut self) {
        self.is_flagged = true;
    }
    pub fn unflag(&mut self) {
        self.is_flagged = false;
    }
    pub fn is_mine(&self) -> bool {
        return self.is_mine;
    }
    pub fn set_local_mines(&mut self, local_mine_count: usize) {
        self.local_mines = local_mine_count;
    }
}
