use core::panic;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

pub fn run() {
    run_example1();
    test_cacher();
}

fn run_example1() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout1(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating sloooowlyyyy....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout1(intensity: u32, random_value: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_value == 3 {
            println!("Take a break today");
        } else {
            println!(
                "Today, run for {} minutes",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn generate_workout2(intensity: u32, random_value: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next do {} situps!", expensive_closure(intensity));
    } else {
        if random_value == 3 {
            println!("Take a break today");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout3(intensity: u32, random_value: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next do {} situps!", expensive_result.value(intensity));
    } else {
        if random_value == 3 {
            println!("Take a break today");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher_2<T, V>
where
    T: Fn(V) -> V,
{
    calculation: T,
    value: Option<V>,
}

impl<T, V> Cacher_2<T, V>
where
    T: Fn(V) -> V,
    V: Clone,
{
    fn new(calculation: T) -> Cacher_2<T, V> {
        Cacher_2 {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: V) -> V {
        match self.value.clone() {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v.clone());
                v
            }
        }
    }
}

fn test_cacher() {
    let mut x = Cacher_2::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let value = x.value(6u32);
    println!("Value is {}", &value);

    let arg1 = String::from("string argument");

    let mut y = Cacher_2::new(|str| format!("{} {}", "This is a test string plus value:", str));

    let v2 = y.value(arg1);
    println!("cacher for strings: {}", &v2);
}

struct Cacher_3<T, V>
where
    T: Fn(V) -> V,
    V: Eq + Hash,
{
    calculation: T,
    map: HashMap<V, V>,
}

impl<T, V> Cacher_3<T, V>
where
    T: Fn(V) -> V,
    V: Eq + Hash + Clone,
{
    fn new(calculation: T) -> Cacher_3<T, V> {
        Cacher_3 {
            calculation,
            map: HashMap::new(),
        }
    }
    fn value(&mut self, arg: V) -> &V {
        if self.map.contains_key(&arg) {
            if let Some(v) = self.map.get(&arg) {
                &v
            } else {
                panic!("");
            }
        } else {
            let v = (self.calculation)(arg.clone());
            self.map.insert(arg.clone(), v);
            self.map.get(&arg).unwrap()
        }
    }
}
