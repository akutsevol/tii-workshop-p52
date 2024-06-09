use std::thread;
use std::sync::Arc;

fn map_sum1<F, const N: usize>(values: Vec<u32>, mapper: F) -> u64
where
    F: Fn(u32) -> u64 + Send + 'static,
{
    let chunk_size = values.len() / N;
    let mut handles = vec![];

    let mapper = Arc::new(mapper); // Wrap the closure in an Arc

    for chunk in values.chunks(chunk_size) {
        let mapper = Arc::clone(&mapper); // Clone Arc to share ownership
        let handle = thread::spawn(move || {
            chunk.iter().map(|&v| mapper(v)).sum::<u64>()
        });
        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}


fn test() {
    // let h1 = std::thread::spawn(move || {
    //     // let a: u32  = buf.iter().sum();
    //     let s: u32 = rc1.iter().sum();
    //     std::thread::sleep(Duration::from_secs(1));
    //     println!("Hi from thread {}!", s);
    //     // buf[10];
    //     2u32
    // });
}