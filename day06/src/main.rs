use std::fs::File;
use std::io;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

struct Fish {
    ages: [usize; 9],
}

impl Fish {
    fn read(path: &str) -> io::Result<Self> {
        let mut contents = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;
        let err = io::Error::new(io::ErrorKind::InvalidData, "Inproper input file");
        contents.trim().parse().map_err(|_| err)
    }

    fn count(&self) -> usize {
        self.ages.iter().sum()
    }

    fn simulate(&mut self, n: usize) {
        for _ in 0..n {
            self.ages.rotate_left(1);
            self.ages[6] += self.ages[8];
        }
    }
}

impl FromStr for Fish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ages = [0; 9];
        for s in s.split(',') {
            let i = usize::from_str_radix(s, 10)?;
            ages[i] += 1;
        }
        Ok(Self { ages })
    }
}

fn main() -> io::Result<()> {
    let mut fish = Fish::read("input")?;
    fish.simulate(256);
    println!("{}", fish.count());
    Ok(())
}
