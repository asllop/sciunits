use sciunits::*;

fn main() {
    let f = 1 / Time::si(9.99);
    println!("Frequency = {}", f);
    let t =  1 / f;
    println!("Time = {}", 1 / t);
    println!("From rad/s = {}", Frequency::from_rads(15.0));
    println!("From rpm = {}", Frequency::from_rpm(150.0));
    println!("{}", Time::si(50.5) + Time::si(5.0));
    println!("{}", Time::si(50.5) * Time::si(5.0));

    println!("{}", Frequency::from_rads(-15.0) + Frequency::si(1.0));
}
