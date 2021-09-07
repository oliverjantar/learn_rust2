pub fn run() {
    simple();
}

fn simple() {
    let x = vec![1, 3, 55, 6, 88];
    let x_iter = x.iter();

    for item in x_iter {
        println!("{}", item);
    }
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: i32,
    brand: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

struct Counter {
    count: u8,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/*
Also note that the values we get from the calls to next are immutable references
to the values in the vector. The iter method produces an iterator over immutable
references. If we want to create an iterator that takes ownership of v1 and returns
owned values, we can call into_iter instead of iter. Similarly, if we want to iterate
over mutable references, we can call iter_mut instead of iter.
*/

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn immutable_references() {
        let x = vec![1, 3, 55, 6, 88];
        let mut x_iter = x.iter();

        assert_eq!(x_iter.next(), Some(&1));
        assert_eq!(x_iter.next(), Some(&3));
        assert_eq!(x_iter.next(), Some(&55));
        assert_eq!(x_iter.next(), Some(&6));
        assert_eq!(x_iter.next(), Some(&88));
        assert_eq!(x_iter.next(), None);
    }

    #[test]
    fn owned_values() {
        let x = vec![1, 3, 55, 6, 88];
        let mut x_iter = x.into_iter();

        assert_eq!(x_iter.next(), Some(1));
        assert_eq!(x_iter.next(), Some(3));
        assert_eq!(x_iter.next(), Some(55));
        assert_eq!(x_iter.next(), Some(6));
        assert_eq!(x_iter.next(), Some(88));
        assert_eq!(x_iter.next(), None);
    }

    #[test]
    fn mutable_reference() {
        let s1 = String::from("test");
        let s2 = String::from("some string");

        let mut x = vec![s1, s2]; // s1 and s2 goes out of scope here

        // println!("{}", s1); //borrowed of moved value

        for item in x.iter_mut() {
            item.push_str(" some string");
        }

        assert_eq!(x[0], String::from("test some string"));
        assert_eq!(x[1], String::from("some string some string"));
    }

    #[test]
    fn iterator_adaptor1() {
        let v1 = vec![1, 5, 33, 6];

        let x: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(x, vec![2, 6, 34, 7]);
    }

    #[test]
    fn iterator_adaptor2() {
        let v1 = vec![1, 2, 3, 4, 5];
        let increment_by_3 = |x| x + 3;
        let x: Vec<i32> = v1.iter().map(increment_by_3).collect();
        assert_eq!(x, vec![4, 5, 6, 7, 8]);
    }

    #[test]
    fn filter_shoes_by_size() {
        let shoes = vec![
            Shoe {
                size: 9,
                brand: String::from("Converse"),
            },
            Shoe {
                size: 10,
                brand: String::from("Vans"),
            },
            Shoe {
                size: 10,
                brand: String::from("Adidas"),
            },
        ];

        let filtered_shoes = shoes_in_size(shoes, 10);

        assert_eq!(
            filtered_shoes,
            vec![
                Shoe {
                    size: 10,
                    brand: String::from("Vans"),
                },
                Shoe {
                    size: 10,
                    brand: String::from("Adidas"),
                }
            ]
        );
    }

    #[test]
    fn test_counter() {
        let mut c = Counter::new();

        assert_eq!(Some(1), c.next());
        assert_eq!(Some(2), c.next());
        assert_eq!(Some(3), c.next());
        assert_eq!(Some(4), c.next());
        assert_eq!(Some(5), c.next());
        assert_eq!(None, c.next());
    }
}
