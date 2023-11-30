use crate::error::Error;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    Free,
    Trash,
}

#[derive(Clone, Debug)]
pub struct State {
    x: usize,
    y: usize,
    steps: usize
}

impl State {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            steps: 0
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
    match termination {
        Termination::Steps(i) => {
            if *i == state.steps {
                return true;
            }
        }
        // if i == state.steps break 
        // println!("{}",x),
    };
    return false;
    // todo!()
    // let termination = Termination::Steps(steps)   {
    //     println!("Terminate after {} steps", steps);
    // }
    // return true
    // if 
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
    // todo!();
    // println!("{}",output.into());
    // return String::from("");
    // println!("Output: {:?}", output);
    // return "".to_string();
    // match output {
    //     Output::Position => println!("Helloo"),
    // };
    // match output{
    //     Output::Position => print!("{}",Position),
    // }
    return "abc".to_string()
}

pub(crate) fn step(map: &mut crate::map::Map, state: &mut State, slope: (usize, usize)) -> () {
    // todo!()
    // let (x,y) = &slope.;
    state.steps += 1;
    state.x += slope.0;
    state.y += slope.1;

    let cell = map.get((state.x,state.y));
    match cell {
        Cell::Trash => map.clean((state.x,state.y)),
        Cell::Free => {},

    }

    // println!("Step!!");
    // println!("{} {}",x,y);
    // return ();
    // return (slope[0],slope[1]);
}
