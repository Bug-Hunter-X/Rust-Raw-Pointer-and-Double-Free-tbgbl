fn main() {
    let mut v = Box::new(vec![1, 2, 3]);
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!(" {:?} ", *v); // Safe access via Box

    // Memory is released properly through Box drop
    drop(v); 
} 