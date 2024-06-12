fn main() {
    use p52::{map_sum1, map_sum2, map_sum3, map_sum4};

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = map_sum1::<4>(data, example_map_fn);

    println!("Result [map_sum1] {:?}", result);

    let data = vec![6, 7, 8, 9, 10, 6, 7, 8, 9, 10];
    let result = map_sum2::<4>(data.clone(), example_map_fn);
    println!("Result [map_sum2] {:?}", result);

    let data = vec![1, 32, 13, 4, 5, 6, 7, 8, 9, 10];
    let result = map_sum3::<4>(data.clone(), example_map_fn);
    println!("Result [map_sum3] {:?}", result);

    let data = vec![1, 52, 3, 4, 25, 6, 47, 8, 9, 10];
    let result = map_sum4::<4>(data.clone(), example_map_fn);
    println!("Result [map_sum4] {:?}", result);
}

fn example_map_fn(x: u32) -> u64 {
    (x * 2) as u64
}
