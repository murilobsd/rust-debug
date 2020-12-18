use core::fmt;

#[derive(Debug)]
enum State {
    Red,
    Blue,
    Green,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self {
            Red => 'R',
            Blue => 'B',
            Green => 'G'
        };
        write!(f, "{}", char)
    }
}

fn main() {
    let s: State = State::Red;

    println!("{}", s);
}
