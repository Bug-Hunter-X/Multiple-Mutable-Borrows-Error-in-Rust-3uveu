fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x;
        *y += 1;
    }
    { // Create another new scope
        let z = &mut x;
        *z += 2;
    }
    println!("x = {}", x);
}