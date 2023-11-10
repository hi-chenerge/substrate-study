fn main() {
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
