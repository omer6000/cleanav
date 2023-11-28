use crate::error::Error;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    Free,
    Trash,
}

#[derive(Clone, Debug)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
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
            unimplemented!()
        })
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Free => '.',
            Cell::Trash => 'X',
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
    todo!()
}

pub enum Output {
    Position,
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
            _ => panic!("{} is not supported", text),
        }
    }
}

pub(crate) fn output(output: &Output, end: (crate::map::Map, State)) -> String {
    todo!();
}

pub(crate) fn step(map: &mut crate::map::Map, state: &mut State, slope: (usize, usize)) -> () {
    todo!()
}
