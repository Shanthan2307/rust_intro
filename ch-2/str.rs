fn greet(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let name1 = "Alice"; // &str
    let name2 = String::from("Bob"); // String
    
    greet(name1);  // Works with &str
    greet(&name2); // Works with String (by borrowing as &str)

    //So any string used rust program without any explicit mentioning of string word is &str( If a string is created without explicitly using String, it is most likely a string literal, which means it is of type &str.)
    //And the variable used to create that string is technically a pointer and is just referring it in the memory

    //let s = "Hello, world!"; // This is &str 
    //"Hello, world!" is a string literal, which is stored in the binary’s read-only memory.
     //s is a reference to that string literal, making s a &str.

     //Yes, &str is technically a pointer!

        //More specifically:

        //&str is a reference to a region of memory that stores the string data.

        //It is a fat pointer, meaning it contains both:

        //A pointer to the string data.

        //The length of the string.
}

