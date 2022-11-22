fn main() {
    println!("{:?}", is_prime(8));
}
fn is_prime(x: i64) -> bool {
    // code here
    for i in 1..=x {
        println!("{:?}", i);
    }
    true
}
