
trait Args {
}

impl Args for () {}
impl <T> Args for (T,) {}
impl <T> Args for (T,T,) {}
impl <T> Args for (T,T,T) {}

trait IntoTuple<O: Args>: Args {
    // type Output where Self: Args;
    fn into_tuple(self) -> O;
}

// impl <A: Args> From<Robot> for A {
//     fn from(val: Robot) -> Self {
//         val.into()
//     }
// }

#[derive(Debug)]
enum Robot {
    Walk(String),
    Charge(String),    
}

struct MySelf {}

impl MySelf {
    fn next(self) -> Self { self }
}

impl Robot {
    fn next(self) -> Self {
        match self {
            Robot::Charge(s) => Robot::Walk(s),
            Robot::Walk(s) => Robot::Charge(s),
        }
    }
}

fn main() {
    let mut s = Robot::Walk("w".into());
    let mut n = 0;

    loop {
        s = s.next();

        n += 1;

        if n == 3 { break; }
    }

    println!("{s:?}");
}