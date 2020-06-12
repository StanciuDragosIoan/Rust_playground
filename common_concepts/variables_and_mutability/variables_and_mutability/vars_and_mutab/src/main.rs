// fn main() {
//     println!("Hello, variable immutable!");
// }
fn main() {
    let mut x = 5;
    // let x = 5; //this linethrows error as var is immutable and we try to change it
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
