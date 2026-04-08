struct User {
    first_name: String,
    last_name: String,
    age: u32,
}
fn main() {
    let user = User {
        first_name: String::from("Navneet"),
        last_name: String::from("S K"),
        age: 32,
    };
    println!("{}", user.first_name);
    println!("{}", user.last_name);
    println!("{}", user.age);
}