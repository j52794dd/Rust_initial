// Define a custom struct
struct Counter {
    count: u32,
}

// Implementing the Iterator trait for Counter
impl Iterator for Counter {
    type Item = u32;  // The type of item the iterator yields

    // Implement the `next` method
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None  // End the iteration when count reaches 5
        }
    }
}

// Implementing a constructor for Counter
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

fn main() {
    let mut counter = Counter::new();  // Create a new Counter

    // Using while let to pattern match on the iterator's `next` method
    let Some(item) = counter.next();
    //let Some(&item) = counter.next();

    let mut collection = MyCollection::new();

    while let Some(item) = collection.next() {  // Pattern matching with a reference
        println!("{}", item);
    }
}

struct MyCollection {
    items: Vec<String>,
    index: usize,
}

impl MyCollection {
    fn new() -> MyCollection {
        MyCollection {
            items: vec!["First".to_string(), "Second".to_string(), "Third".to_string()],
            index: 0,
        }
    }
}

impl Iterator for MyCollection {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let result = &self.items[self.index];  // Return a reference to the current item
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

