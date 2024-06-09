use std::arch::x86_64::*;
use std::time::Duration;
use std::fs::File;
use std::env;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use rayon::prelude::*;
use std::sync::Arc;
use std::{cell::RefCell, sync::Mutex};

mod lib;

thread_local! {
    static FOO: RefCell<u32> = RefCell::new(42);
}

fn main () {
    test_threads1();
    test_threads2();
    test_threads3();
    computing_the_dot_product_of_two_large_vectors();
    processing_large_files();
    test_scope();
    test_add_sse2();
    test_add_avx2();
}

fn computing_the_dot_product_of_two_large_vectors () {
    println!("computing_the_dot_product_of_two_large_vectors:");
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![6, 7, 8, 9, 10];

    let dot_product: i32 = vec1.par_iter()
        .zip(vec2.par_iter())
        .map(|(&x, &y)| x * y)
        .sum();

    println!("{}", dot_product); // Prints 130
}

fn processing_large_files () {
    println!("processing_large_files:");
    // Function to read a file into a Vec<String>
    fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    let path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let log_file_name = format!("{path}/src/{}", "large_log_file.log");
    let lines = read_lines(log_file_name).expect("Could not read file");

    let error_count: usize = lines.par_iter()
        .filter(|line| line.contains("LQM-WIFI"))
        .count();

    println!("Number of \"LQM-WIFI\" logs: {}", error_count);
}

fn test_scope() {
    println!("test_scope");
    fn factorial_sequential(n: u128) -> u128 { 
        (1..=n).reduce(|multiple, next| multiple * next).unwrap() 
    } 

    let mut map: Option<HashMap<String, usize>> = None; 
    let mut factorial = 1; 
    let mut other = None; 
    rayon::scope(|s| { 
        s.spawn(|_s| { 
            let iter = 
                (0..10000).enumerate().map(|(a, b)| (format!("index {}", a), b)); 
            map = Some(HashMap::from_iter(iter)); 
        }); 
        s.spawn(|_s| { 
            factorial = factorial_sequential(30); 
        }); 
        s.spawn(|_s| { 
            other = Some("value") 
        }) 
    }); 

    // println!("map {:?}", map.unwrap()); 
    println!("factorial {:?}", factorial); 
    // println!("other {:?}", other); 
} 

fn test_threads1() {
    println!("test_threads1:");
    let buf = vec!(1u32, 2, 3);
    let rc1 = buf.clone();
    let rc2 = buf.clone();

    let h1 = std::thread::spawn(move || {
        // let a: u32  = buf.iter().sum();
        let s: u32 = rc1.iter().sum();
        std::thread::sleep(Duration::from_secs(1));
        println!("Hi from thread {}!", s);
        // buf[10];
        2u32
    });
    let h2 = std::thread::spawn(move || {
        // let b: u32 = buf.iter().map(|&x| x * x).sum();
        let a: u32 = rc2.iter().map(|&x| x * x).sum();
        std::thread::sleep(Duration::from_secs(3));
        println!("Hi from thread {}!", a);
        42u32
    });
    let res = h1.join().expect("Thread 1 panic") + h2.join().expect("Thread 2 panic");

    println!("{res}");
}

fn test_threads2() {
    println!("test_threads2:");
    let buf = Arc::new(vec![1u32, 2, 3]);
    let rc1 = buf.clone();
    println!("size_of_val(&rc1) = {}", std::mem::size_of_val(&rc1));
    let rc2 = buf.clone();
    let h1 = std::thread::spawn(move || {
        // let a: u32  = buf.iter().sum();
        let s: u32 = rc1.iter().sum();
        std::thread::sleep(Duration::from_secs(1));
        println!("Hi from thread {}!", s);
        // buf[10];
        2u32
    });
    let h2 = std::thread::spawn(move || {
        // let b: u32 = buf.iter().map(|&x| x * x).sum();
        let a: u32 = rc2.iter().map(|&x| x * x).sum();
        std::thread::sleep(Duration::from_secs(3));
        println!("Hi from thread {}!", a);
        42u32
    });
    let res = h1.join().expect("Thread 1 panic") + h2.join().expect("Thread 2 panic");

    println!("{res}");
}

fn test_threads3() {
    println!("test_threads3:");
    let buf = Arc::new(Mutex::new(vec![1u32, 2, 3]));
    
    let rc1 = buf.clone();
    let rc2 = buf.clone();
    let h1 = std::thread::spawn(move || {
        {
            let mut guard = rc1.lock().unwrap();
            for value in guard.iter_mut() {
                *value *= 10;
            }
        }
        std::thread::sleep(Duration::from_secs(1));
        println!("Hi from thread 1!");
        2u32
    });
    let h2 = std::thread::spawn(move || {
        {
            let mut guard = rc2.lock().unwrap();
            for value in guard.iter_mut() {
                *value *= 10;
            }
        }
        std::thread::sleep(Duration::from_secs(3));
        println!("Hi from thread 2!");
        42u32
    });
    let res = h1.join().expect("Thread 1 panic") + h2.join().expect("Thread 2 panic");

    println!("{res}");
}

fn test_add_sse2() {
    println!("test_add_sse2:");
    let a: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let b: [u32; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
    let mut c: [u32; 8] = [0; 8]; // Initialize c with 
    unsafe {
        add_sse2(&a, &b, &mut c);
    }
    println!("Result: {:?}", c);
}

fn test_add_avx2() {
    println!("test_add_avx2:");
    let a: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let b: [u32; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
    let mut c: [u32; 8] = [0; 8]; // Initialize c with 
    unsafe {
        add_avx2(&a, &b, &mut c);
    }
    println!("Result: {:?}", c);
}

#[target_feature(enable = "sse2")]
unsafe fn add_sse2(a: &[u32; 8], b: &[u32; 8], c: &mut [u32; 8]) {
    let a1: __m128i = _mm_loadu_si128(a.as_ptr().cast());
    let b1: __m128i = _mm_loadu_si128(b.as_ptr().cast());
    let res1: __m128i = _mm_add_epi32(a1, b1);
    _mm_storeu_si128(c.as_mut_ptr().cast(), res1);

    let a2: __m128i = _mm_loadu_si128(a.as_ptr().add(4).cast());
    let b2: __m128i = _mm_loadu_si128(b.as_ptr().add(4).cast());
    let res2: __m128i = _mm_add_epi32(a2, b2);
    _mm_storeu_si128(c.as_mut_ptr().add(4).cast(), res2);
}

#[target_feature(enable = "avx2")]
unsafe fn add_avx2(a: &[u32; 8], b: &[u32; 8], c: &mut [u32; 8]) {
    let a: __m256i = _mm256_loadu_si256(a.as_ptr().cast());
    let b: __m256i = _mm256_loadu_si256(b.as_ptr().cast());
    let res: __m256i = _mm256_add_epi32(a, b);
    _mm256_storeu_si256(c.as_mut_ptr().cast(), res);
}
