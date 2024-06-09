use std::thread;
use std::sync::Arc;

pub fn map_sum1<const N: usize>(data: Vec<u32>, map_fn: fn(u32) -> u64) -> u64 {
    let data_len = data.len();
    let chunk_size = (data_len + N - 1) / N; // Calculate chunk size for each thread

    let data = Arc::new(data);
    let mut handles = vec![];

    for i in 0..N {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(data_len);

            let mut sum = 0u64;
            for j in start..end {
                sum += map_fn(data[j]);
            }
            sum
        });
        handles.push(handle);
    }

    let mut total_sum = 0u64;
    for handle in handles {
        total_sum += handle.join().unwrap();
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_map_fn(x: u32) -> u64 {
        (x * 2) as u64
    }

    #[test]
    fn test_map_sum1() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = map_sum1::<4>(data, example_map_fn);
        assert_eq!(result, 110); // (1*2 + 2*2 + 3*2 + ... + 10*2) = 110
    }
}
