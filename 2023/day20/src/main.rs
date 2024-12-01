fn main() {
    println!("Hello, world!");
}


enum Pulse {
    High(String),
    Low(String),
}

trait Module {
    fn process(&mut self, pulse: Pulse) -> 
