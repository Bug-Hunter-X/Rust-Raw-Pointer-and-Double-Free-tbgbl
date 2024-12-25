fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!( "{:?}", v); // v is now [4, 2, 3]

    // Now let's try to free the memory
    drop(v); //This is not enough to handle the memory properly
    // The memory pointed to by 'ptr' is still accessible, which is very dangerous, this is a double free error
}