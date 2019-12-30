use std::fmt::Debug;

#[derive(Debug)]
enum Coor {
    X(i32),
    Y(i32),
}

#[derive(Debug)]
struct Particle {
    position: (Coor,Coor),
    velocity: (Coor,Coor),
    acceleration: (Coor,Coor)
}
trait Logger {
    fn log(&self);
}

impl Logger for Particle {
    fn log(&self){
        println!("x = {:?}, y = {:?}",&self.position.0, &self.position.1);
        println!("{:?}",&self)
    }
}
fn main() {
    // structs enums and traits
    let particle_one: Particle = Particle{
        position: (Coor::X(20), Coor::Y(300)),
        velocity: (Coor::X(21), Coor::Y(309)),
        acceleration: (Coor::X(41), Coor::Y(11)),
    };

    let p_one_x = match particle_one.position.0 {
        Coor::X(value1) => value1,
        Coor::Y(value2) => value2,
    };

    println!("{}", p_one_x);
    particle_one.log();

    // generic types and lifetimes
    fn longest <'a> (word_one: &'a str, word_two: &'a str) -> &'a str {
        if word_one < word_two {
            word_two
        } else {
            word_one
        }
    };

    let longest = longest("hi", "there");
    println!("{}",longest)

}
