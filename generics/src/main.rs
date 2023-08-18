fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 76, 22, 32];
    println!("The largest number is {}", largest(&number_list));
    println!("The number list is {:?}", number_list);
}
