fn main() {
    println!("{:?}", is_prime(8));
}
fn is_prime(x: i64) -> bool {
    // code here
    let mut counter = 0;
    for i in 1..=x {
        println!("{:?}", i);
        if x % i == 0 {
            counter += 1;
        }
    }
    if counter == 2 {
        true
    } else {
        false
    }
}
