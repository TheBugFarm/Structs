struct rect {
    width: u32,
    height: u32,
}
impl rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self)-> u32 {
        2 * (self.width + self.height)
    }
}
fn main() {
    let rect1 = rect {
        width: 50,
        height: 40,
    };
    println!("{}", rect1.area());
    println!("{}", rect1.perimeter());
    // println!("{}", rect::debug());
}