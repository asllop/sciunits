use sciunits::*;

fn main() {
    let f = 1.0 / Time::si(9.99);
    println!("Frequency = {}", f);
    let t =  1.0 / f;
    println!("Time = {}", 1.0 / t);
    println!("From rad/s = {}", Frequency::from_rads(15.0));
    println!("From rpm = {}", Frequency::from_rpm(150.0));
}
