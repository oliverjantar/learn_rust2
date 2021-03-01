pub fn run() {
    let number_list = vec![45, 7, 33, 2323, 667, 88];

    let char_list = vec!['f', 'd', 'j', 'v'];

    let largest_number = largest_i32(&number_list);
    println!("{}", largest_number);

    let lrg_char = largest_char(&char_list);
    println!("{}", lrg_char);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn make_a_point() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.3, y: 2.5 };

    let p3 = Point2 { x: 3.2, y: 32 };
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    //in this case mixup takes completely different implementation of point other than <T,U>
    //that's why we need to bring <V,W>
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
