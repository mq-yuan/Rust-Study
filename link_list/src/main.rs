#[derive(Debug)]
enum List {
    Nil,
    Cons(u8, Box<List>),
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn len(&self) -> u8 {
        match *self {
            List::Nil => 0,
            List::Cons(_, ref tail) => 1 + tail.len(),
        }
    }

    fn prepend(self, elem: u8) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn stringfly(&self) -> String {
        match *self {
            List::Nil => format!("Nil"),
            List::Cons(head, ref tail) => {
                format!("{} {}", head, tail.stringfly())
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(2);
    list = list.prepend(4);
    list = list.prepend(5);
    println!("{}, Length is {}", list.stringfly(), list.len());
}
