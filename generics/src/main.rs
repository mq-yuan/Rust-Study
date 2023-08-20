// Result and Option is useful
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    // Yu can use T and U for different type
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

// fn display_array<T: std::fmt::Debug>(arr: &[T]) {
//     println!("The array is {:?}", arr);
// }
//
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("The array is {:?}", arr);
}

fn main() {
    let number_list = vec![34, 50, 76, 22, 32];
    let point = Point { x: 1.2, y: 1.1 };
    let arr = [3; 5];
    display_array(arr);
    println!(
        "point to origin distance = {}",
        point.distance_from_origin()
    );
    let point1 = Point {
        x: "hello",
        y: "world",
    };
    let point = point.mixup(point1);
    println!("The largest number is {}", largest(&number_list));
    println!("The number list is {:?}", number_list);
    println!("point = ({}, {})", point.x(), point.y());
}
