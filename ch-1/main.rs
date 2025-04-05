// Primitive data types
// int , float , char , bool 

// Rust is a statically typed language, which means that the type of a variable must be known at compile time.

// intergers
// i8 , i16 , i32 , i64 , i128 , isize :signed integers (positive and negative numbers)
// u8 , u16 , u32 , u64 , u128 , usize : unsigned integers (only positive numbers)
// isize and usize depend on the architecture of the machine (32 or 64 bit).

fn main(){
    let a: i32 = -1560;
    let b: u32 = 1856;

    println!("signed integer: {}",a);
    println!("unsigned integer:{}",b);

    // floats [Floating Point types]
    // f32 , f64
    let pi: f64 = 3.14;
    println!("Values of pi:{}",pi);
    //bool true, False
    let is_snowing: bool =true;
    println!("is it snowing? {}",is_snowing);
    
    //character type - char
    let letter: char = 'a';
    println!("first letter of alphabet: {}",letter);

}