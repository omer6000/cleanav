use crate::error::Error;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    Free, // .
    Trash, // X
    Buoy, // O
}

#[derive(Clone, Debug)]
pub struct State {
    x: usize,
    y: usize,
    steps: usize,
    buoy: u16, //count
    free: u16, //count
    trash: u16, //count
    lastcell: Cell
}

impl State {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            steps: 0,
            buoy: 0,
            free: 0,
            trash: 0,
            lastcell: Cell::Free,
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(if c == '.' {
            Cell::Free
        } else if c == 'X' {
            Cell::Trash
        } else {
            Cell::Buoy
        })
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Free => '.',
            Cell::Trash => 'X',
            Cell::Buoy => 'O',
        }
    }
}

pub enum Termination {
    Steps(usize),
    Symbol(char),
    Position((usize,usize)),
}

impl From<String> for Termination {
    fn from(text: String) -> Self {
        if text.to_lowercase().starts_with("s") {
            Termination::Symbol(text.chars().nth(2).unwrap())
        }
        else if text.to_lowercase().starts_with("p") {
            let x = text[2..3].parse().unwrap();
            let y = text[4..5].parse().unwrap();
            Termination::Position((x,y))
        }
        else {
            Termination::Steps(text.parse().unwrap())
        }
    }
}

pub(crate) fn terminal(termination: &Termination, map: &crate::map::Map, state: &State) -> bool {
    match termination {
        Termination::Steps(i) => {
            if *i == state.steps {
                return true;
            }
        }
        Termination::Position((x,y)) => {
            if *x == state.x && *y == state.y {
                return true
            }
        }
        Termination::Symbol(s) => {
            let term_cell = Cell::try_from(*s).unwrap();
            let lastcell: Cell;
            if state.steps == 0 {
                lastcell = map.get((0,0));
            }
            else {
                lastcell = state.lastcell;
            }
            if term_cell == lastcell {
                return true;
            }
        }
    };
    return false;
}

pub enum Output {
    Position,
    Steps,
    TerminalSymbol,
    DistinctSymbols,
    SymbolCount(Cell),
    // Trash,
    // Free,
    // Buoy,
}

impl FromStr for Output {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Output::from(s))
    }
}

impl<'a, S> From<S> for Output
where
    S: Into<&'a str>,
{
    fn from(text: S) -> Self {
        let text: &str = text.into();
        match text.to_lowercase().as_str() {
            "p" => Output::Position,
            "n" => Output::Steps,
            "s" => Output::TerminalSymbol,
            "d" => Output::DistinctSymbols,
            "c,x" => Output::SymbolCount(Cell::Trash),
            "c,." => Output::SymbolCount(Cell::Free),
            "c,o" => Output::SymbolCount(Cell::Buoy),
            _ => panic!("{} is not supported", text),
        }
    }
}

pub(crate) fn output(output: &Output, end: (crate::map::Map, State)) -> String {
    match output {
        Output::Steps => return end.1.steps.to_string(),
        Output::DistinctSymbols => {
            let mut distinct: u8 = 0;
            if end.1.buoy != 0 {
                distinct += 1;
            }
            if end.1.free != 0 {
                distinct += 1;
            }
            if end.1.trash != 0 {
                distinct += 1;
            }
            return distinct.to_string();
        },
        Output::TerminalSymbol => {
            // match end.1.lastcell {
            match end.0.get((end.1.x,end.1.y)) {
                Cell::Free => return '.'.to_string(),
                Cell::Trash => return 'X'.to_string(),
                Cell::Buoy => return 'O'.to_string(),
            }
        }
        Output::Position => return format!("({}, {})", end.1.x,end.1.y),
        Output::SymbolCount(cell) => {
            match *cell {
                Cell::Buoy => return end.1.buoy.to_string(),
                Cell::Free => return end.1.free.to_string(),
                Cell::Trash => return end.1.trash.to_string(),
            }
        }

    };
}

fn process_cell(state: &mut State,map: &mut crate::map::Map) {
    let cell = map.get((state.x,state.y));
    match cell {
        Cell::Trash => {
            map.clean((state.x,state.y));
            state.trash += 1;
            state.lastcell = Cell::Trash;
        },
        Cell::Free => {
            state.free += 1;
            state.lastcell = Cell::Free;
        },
        Cell::Buoy => {
            state.buoy += 1;
            state.lastcell = Cell::Buoy;
        },
    };
}


pub(crate) fn step(map: &mut crate::map::Map, state: &mut State, slope: (usize, usize)) -> () {
    if state.steps == 0 { //dealing with step 0
        process_cell(state, map);
    }
    state.steps += 1;
    state.x = (state.x + slope.0) % map.width();
    state.y = (state.y + slope.1) % map.height();

    process_cell(state, map);

}
