fn main() {
    use p52::map_sum1;

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = map_sum1::<4>(data, example_map_fn);

    println!("Result {:?}", result);
}

fn example_map_fn(x: u32) -> u64 {
    (x * 2) as u64
}
