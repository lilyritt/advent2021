use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(dir: &str) -> Result<Self, Self::Err> {
        match dir {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

struct Command {
    dir: Direction,
    amount: u32,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let split = cmd.split_once(' ').ok_or(())?;
        let dir = split.0.parse()?;
        let amount = split.1.parse().map_err(|_| ())?;
        Ok(Self { dir, amount })
    }
}

struct Position {
    x: u32,
    y: u32,
    aim: u32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn run(&mut self, cmd: &Command) {
        match cmd.dir {
            Direction::Forward => {
                self.x += cmd.amount;
                self.y += self.aim * cmd.amount;
            }
            Direction::Down => self.aim += cmd.amount,
            Direction::Up => self.aim -= cmd.amount,
        }
    }

    fn product(&self) -> u32 {
        self.x * self.y
    }
}

fn read_commands(path: &str) -> io::Result<Vec<Command>> {
    let file = File::open(path)?;
    let mut cmds = Vec::new();
    let buffer = BufReader::new(file);
    for line in buffer.lines() {
        let cmd = line?.parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid line"))?;
        cmds.push(cmd);
    }
    Ok(cmds)
}

fn main() -> io::Result<()> {
    let mut pos = Position::new();
    let cmds = read_commands("input")?;
    for cmd in cmds {
        pos.run(&cmd);
    }
    println!("{}", pos.product());
    Ok(())
}
