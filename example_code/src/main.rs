
// Euler1 in Rust
// If we list all the natural numbers below 10 that are multiples of 3 or 5
// we get 3,5,6 and 9.The sum of these multiples is 23. Find the sum of all
// multiples of 3 or 5 below 1000.

fn euler1(n: i32) -> i32 {
    let mut retval: i32 = 0;

    for i in 0.. {
        if i >= n {
            break;
        }
        if i % 3 == 0 || i % 5 == 0 {
            retval += i;
        }
    }
    retval
}

fn main() {
    println!("Euler1 = {}", euler1(1000));
}
