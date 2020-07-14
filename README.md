# Learning Rust Part 3: Vectors

In this session our goal is to create some 3-dimensional vectors and add operations:
* Sum two vectors together using the ```+``` operator
* Substract two vectors using the ```-``` operator

To achieve this, we should have some knowledge of the following:
* How to define ~~classes~~ *structs* and instantiate them
* How to separate the project into multiple files
* How to overload operators
* How to write simple tests

## Creating the Vec3 struct

Our 3-dimensional vector consists of 3 double-precision floating point values (x, y and z):

```rust
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}
```
It would be also nice to be able to print the vector in a way we like. This can be done by implementing ```fmt::Display```:
```rust
use core::fmt;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
```

Now we can create an instance of ```Vec3``` and print its coordinates to the standard output:
```rust
fn main() {
    let vector = Vec3 {
        x: 1.0, 
        y: 2.0, 
        z: 3.0
    };

    println!("Vector coordinates are {}", vector);
    // Output:
    // Vector coordinates are (1, 2, 3)
}
```

> 💡 Note: The ```fmt::Display``` formatting looks a bit tricky. It is fortunately covered early in [Rust by Example](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html).

## Modules

We don't want everything to be contained in a single file, and in the future it will be useful to limit the visibility of struct members and functions.

Let's move the ```Vec3``` struct and its functions to file ```vector.rs```. While we´re at it, let's also add a public constructor and limit the visibility of the member variables, although they will probably need to be visible soon.

```rust
// vector.rs
use core::fmt;

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
```
To use ```Vec3``` in the main program, add a reference with the ```mod``` keyword.
```rust
// main.rs
mod vector;
use vector::Vec3;

fn main() {
    let vector = Vec3::new(1.0, 2.0, 3.0);

    println!("Vector coordinates are {}", vector);
    // Output:
    // Vector coordinates are (1, 2, 3)
}
```

## Adding and substracting vectors

To add vectors together, we must simply create a new vector and add x-axis values together, then y-axis values and so on:

```rust
// vector.rs
    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
```
To test this in the main program, let's create two vectors and add them together:
```rust
fn main() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(0.5, 0.3, 0.2);

    println!("Vector 1 value is {}", vector1);
    println!("Vector 2 value is {}", vector2);
    println!("Vector addition result is {}", vector1.add(vector2));
```
```
> cargo run

Vector 1 value is (1, 2, 3)
Vector 2 value is (0.5, 0.3, 0.2)
Vector addition result is (1.5, 2.3, 3.2)
```
That was easy! Now, let's add substraction and test that:
```rust
// vector.rs
    pub fn sub(&self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z
        }
```
```rust
fn main() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(0.5, 0.3, 0.2);

    println!("Vector 1 value is {}", vector1);
    println!("Vector 2 value is {}", vector2);
    println!("Vector addition result is {}", vector1.add(vector2));
    println!("Vector substraction result is {}", vector1.sub(vector2));
}
```
Uh oh. Now we have some squiggly lines in the editor. Trying to compile, we get the following error:
```
error[E0382]: use of moved value: `vector2`
  --> src\main.rs:11:63
   |
6  |     let vector2 = Vec3::new(0.5, 0.3, 0.2);
   |         -------- move occurs because `vector2` has type `vector::Vec3`, which does not implement the `Copy` trait
...
10 |     println!("Vector addition result is {}", vector1.add(vector2));
   |                                                           -------- value moved here
11 |     println!("Vector substraction result is {}", vector1.sub(vector2));
   |                                                               ^^^^^^^^ value used here after move

error: aborting due to previous error
```
It seems that ```vector1``` takes ```vector2``` without returning it. Instead, it should only *borrow* it for a while. That can be easily solved with a few strategically placed ampersands (```&```):
```rust
// vector.rs
    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
```

```rust
fn main() {
    let vector1 = Vec3::new(1.0, 2.0, 3.0);
    let vector2 = Vec3::new(0.5, 0.3, 0.2);

    println!("Vector 1 value is {}", vector1);
    println!("Vector 2 value is {}", vector2);
    println!("Vector addition result is {}", vector1.add(&vector2));
    println!("Vector substraction result is {}", vector1.sub(&vector2));
}
```

Now the code compiles again, and we get correct results.
```
Vector 1 value is (1, 2, 3)
Vector 2 value is (0.5, 0.3, 0.2)
Vector addition result is (1.5, 2.3, 3.2)
Vector substraction result is (0.5, 1.7, 2.8)
```

### Overloading operators

Using the ```vector1.add(&vector2)``` syntax works, but is kind of clunky. We can make it a lot cleaner by importing ```std::ops``` and implementing the ```Add``` and ```Sub``` functions:

```rust
// vector.rs
use std::ops::{Add, Sub};
...
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
```
Now the print statements in the main program can written in a cleaner way:
```rust
    println!("Vector addition result is {}", &vector1 + &vector2);
    println!("Vector substraction result is {}", &vector1 - &vector2);
```

### Borrowing vs copying

Our ```Vec3``` structs are tiny, which means that the difference in performance between copying and borrowing is negligible (please do correct me if I'm wrong!). By implementing ```Copy``` for our struct, we can get rid of the superfluous ampersands and finally use the vectors as intended.

Rust makes this really easy: just add a ```#[derive]``` macro and the vectors are copied around instead of passed by reference. Remember to remove the extra ampersands:

```rust
#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + other.x, 
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
```

The final print statements in their minimalistic glory:
```rust
    println!("Vector addition result is {}", vector1 + vector2);
    println!("Vector substraction result is {}", vector1 - vector2);
```

## Tests
We have already implemented a bunch of operations, so it is about time to write our first tests. Rust has a built-in testing framework, which is nice. However it doesn't contain a comparison macro for floating point equality, so we have to import that from ```crates.io```.

```rust
// vector.rs

#[cfg(test)]
mod tests {
    use assert_approx_eq::*;
    use super::*;

    #[test]
    fn addition() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let sum = vector1 + vector2;
        assert_approx_eq!(5.0, sum.x, 0.001);
        assert_approx_eq!(7.0, sum.y, 0.001);
        assert_approx_eq!(9.0, sum.z, 0.001);
    }

    #[test]
    fn substraction() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        let sub = vector1 - vector2;
        assert_approx_eq!(-3.0, sub.x, 0.001);
        assert_approx_eq!(-3.0, sub.y, 0.001);
        assert_approx_eq!(-3.0, sub.z, 0.001);
    }
}
```

With the VS Code plugin, you can run a single test by clicking the *Run test* link, which appears under ```#[test]```. There is also a handy extension called *Rust Test Explorer* ([swellaby.vscode-rust-test-adapter](https://marketplace.visualstudio.com/items?itemName=swellaby.vscode-rust-test-adapter)), which will show all tests in the side bar.

> 💡 Note: You can write unit tests for private functions here, too! Some say that it is a bad practice, some don't. If you don't want to test your privates, simply don't do it.

## Macros

The approximate comparison takes 3 assertations for a single vector to compare all the member values. Why not compare a vector directly with an expected result, which is also a vector?

We can do that by writing our own assertation macro:
```rust
    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected.x, $actual.x, tolerance);
            assert_approx_eq!($expected.y, $actual.y, tolerance);
            assert_approx_eq!($expected.z, $actual.z, tolerance);
        }
    }
```
See what we did there? You can invoke macros *within macros*. This quick and dirty approach won't win you any points in a style competition, but it'll do for now:

```rust
    #[test]
    fn multiplication() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let actual = 5.0 * vector1;
        let expected = Vec3::new(5.0, 10.0, 15.0);
        assert_vec3_equal!(expected, actual);
    }
```

## The rest of the owl

The rest of Chapter 3 is just writing all the other operations for ```Vec3``` (negation, multiplication, dot product and so on).