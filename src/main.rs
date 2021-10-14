use sciunits::*;

fn main() {
    let f = 1.0 / Time::new(9.99);
    println!("Frequency = {}", f);
    let t =  1.0 / f;
    println!("Time = {}", 1.0 / t);
}
