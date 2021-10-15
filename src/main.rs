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

    let a = array!(Time::si(1.0), Time::si(2.0), Time::si(3.0), Time::si(4.0));
    let a2 = a.clone();
    let b = vec!(2.0, 2.0, 2.0, 2.0);
    let c = a * b;
    println!("{}", c);
    println!("{}", 1 / a2);
}
