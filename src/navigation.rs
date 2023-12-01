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
}

impl From<String> for Termination {
    fn from(text: String) -> Self {
        Termination::Steps(text.parse().unwrap())
    }
}

pub(crate) fn terminal(termination: &Termination, map: &crate::map::Map, state: &State) -> bool {
    match termination {
        Termination::Steps(i) => {
            if *i == state.steps {
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
    Trash,
    Free,
    Buoy,
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
            "c,x" => Output::Trash,
            "c,." => Output::Free,
            "c,o" => Output::Buoy,
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
            match end.1.lastcell {
                Cell::Free => return '.'.to_string(),
                Cell::Trash => return 'X'.to_string(),
                Cell::Buoy => return 'O'.to_string(),
            }
        }
        Output::Position => return format!("({},{})", end.1.x,end.1.y),
        Output::Trash => return end.1.trash.to_string(),
        Output::Free => return end.1.free.to_string(),
        Output::Buoy => return end.1.buoy.to_string(),

    };
}

pub(crate) fn step(map: &mut crate::map::Map, state: &mut State, slope: (usize, usize)) -> () {
    state.steps += 1;
    state.x = (state.x + slope.0) % map.width();
    state.y = (state.y + slope.1) % map.height();

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
