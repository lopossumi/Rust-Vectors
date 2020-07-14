mod vector;
use vector::Vec3;

fn main() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(0.5, 0.3, 0.2);

    println!("Vector 1 value is {}", vector1);
    println!("Vector 2 value is {}", vector2);
    println!("Vector addition result is {}", vector1 + vector2);
    println!("Vector substraction result is {}", vector1 - vector2);
}