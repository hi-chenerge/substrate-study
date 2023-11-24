fn main() {
    // test_bubble_sort();
    // test_traffic_light();
    // test_sum();

    test_area();
}

fn test_area() {
    let c = Circle { r: 2.0 };
    area(&c);

    let r = Rectangle { w: 3.0, h: 2.0 };
    area(&r);

    let t = Triangle { b: 3.0, h: 2.0 };
    area(&t);
}

pub trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        // πr²
        self.r * self.r * 3.14
    }
}

struct Rectangle {
    w: f64,
    h: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

struct Triangle {
    b: f64,
    h: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // S = 1/2 × b × h
        self.b * self.h * 0.5
    }
}

fn area<T>(t: &T) where T: Shape {
    println!("{}", t.area());
}

fn test_sum() {
    let list1 = &[1, 2, 3];
    println!("{:?}", sum(list1));

    let list2 = &[u32::MAX, 1];
    println!("{:?}", sum(list2));
}

fn sum(list: &[u32]) -> Option<u32> {
    let mut result: u32 = 0;
    let err_result: Option<u32> = None;

    for i in list {
        if let Some(sum) = result.checked_add(*i) {
            result = sum;
        } else {
            return err_result;
        }
    }

    Some(result)
}


fn test_traffic_light() {
    println!("{}", TrafficLight::GREEN.count_down());
    println!("{}", TrafficLight::RED.count_down());
    println!("{}", TrafficLight::YELLOW.count_down());
}

enum TrafficLight {
    RED,
    YELLOW,
    GREEN,
}

trait LightFun {
    fn count_down(&self) -> u8;
}

impl LightFun for TrafficLight {
    fn count_down(&self) -> u8 {
        match self {
            TrafficLight::RED => { 60 }
            TrafficLight::YELLOW => { 3 }
            TrafficLight::GREEN => { 45 }
        }
    }
}

fn test_bubble_sort() {
    let mut list1 = vec![10, 100, 30, 20, 50];
    let mut list2 = vec![10.1, 100.3, 30.5, 20.9, 50.6];

    bubble_sort(&mut list1);
    bubble_sort(&mut list2);

    println!("list1 sorted result:");
    for x in &list1 {
        println!("{}", *x)
    }

    println!("list2 sorted result:");
    for x in &list2 {
        println!("{}", *x)
    }
}

fn bubble_sort<T>(list: &mut [T]) where T: PartialOrd {
    let len = list.len();

    for i in 0..len {
        for j in 0..len - 1 - i {
            // 如果后一个小，则交换到前面
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}
