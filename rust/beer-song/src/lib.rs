use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum Bottles {
    Zero(i32),
    One(i32),
    MoreThanOne(i32),
}

impl Bottles {
    pub fn action(&self) -> String {
        use Bottles::*;
        match self {
            &Zero(_) => "Go to the store and buy some more".to_string(),
            &One(_) => "Take it down and pass it around".to_string(),
            &MoreThanOne(_) => "Take one down and pass it around".to_string(),
        }
    }

    pub fn from_i32(bottle_cnt: i32) -> Bottles {
        use Bottles::*;
        match bottle_cnt {
            x if x < 0 => MoreThanOne(99),
            x if x > 1 => MoreThanOne(x),
            x if x == 1 => One(x),
            _ => Zero(0),
        }
    }
}

impl Display for Bottles {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Bottles::*;
        match self {
            &Zero(_) => write!(f, "No more bottles of beer"),
            &One(_) => write!(f, "1 bottle of beer"),
            &MoreThanOne(n) => write!(f, "{} bottles of beer", n),
        }
    }
}


pub fn verse(bottle_cnt: i32) -> String {
    let bottles = Bottles::from_i32(bottle_cnt);
    let bottles_lower = bottles.to_string().to_lowercase();
    let action = bottles.action();
    let next = Bottles::from_i32(bottle_cnt - 1)
        .to_string()
        .to_lowercase();

    format!("{} on the wall, {}.\n{}, {} on the wall.\n",
            bottles,
            bottles_lower,
            action,
            next)
}

pub fn sing(start: i32, end: i32) -> String {
    let proper_range = (end..start + 1).rev();

    proper_range.map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}