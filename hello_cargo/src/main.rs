#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
    }

}

enum Greeting {
    Hi(String),
    Swear(String)
}

impl Greeting {
    fn greet(&self) {
        match self {
            Greeting::Hi(message) => println!("{}", message),
            Greeting::Swear(swear) => println!("{}", swear),
        }
        
    }
}

struct Holder<T> {
    elm: T,
}

pub trait Stringer {
    fn to_string(&self) -> &String;
}

fn largest<T: PartialOrd>(xs: &[T]) -> &T {
    let mut crt_max = &xs[0];
    for x in &xs[1..] {
        if crt_max < x {
            crt_max = x
        }
    }
    crt_max
}

impl Stringer for Holder<String> {
    fn to_string(&self) -> &String {
        &self.elm
    }
}

#[derive(Debug)]
enum Either<A, B> {
    A(A),
    B(B),
}

use std::fmt;
impl<A,B> fmt::Display for Either<A, B>
    where   A: fmt::Display,
            B: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Either::A(a) => write!(f, "A({})", a),
            Either::B(b) => write!(f, "B({})", b),
        }
    }
}

fn printago(t: &impl std::fmt::Debug) {
    println!("{:?}", t);
}

impl<A,B> Either<A,B> {
    fn yehoi(&self) -> bool {
        true
    }
}

struct Wrapper<A>(Vec<A>);
impl<A: fmt::Display> fmt::Display for Wrapper<Either<A,A>> {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in &self.0 {
            write!(f, "{}", i)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.name)?;
        write!(f, "{}", self.age)
    } 
}

struct Name(String);
struct FullName(Name, Name);

fn main() {
    let radu = FullName(
        Name(String::from("radu")),
        Name(String::from("stoenescu"))
    );

    let FullName(Name(_), Name(familie)) = radu;
    println!("{}", familie);
}
