// Compound Data types
// arrays,tuples, slices and strings (slice string)

// array
fn main(){
    // arrays
    let numbers :[i32;5] = [1,2,3,4,5];
    println!("number Array: {:?}",numbers);
    let fruits : [&str ; 3] = ["apple","banana","Orange"];
    println!("fruits array: {:?}",fruits);
    println!("fruits 1st element is {}",fruits[0]);
    println!("fruits 2nd element is {}",fruits[1]);
    println!("fruits 3st element is {}",fruits[2]);

    // tuples

    let human :(String,i32,bool) = ("gargi".to_string(),19,true);
    println!("Human tuple: {:?}",human);
    let my_mix_tuples = ("kratos",23,true,[1,2,3,4,5,6]);
    println!("my mix tuples {:?}",my_mix_tuples);
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("number slices is {:?}", number_slices);
    let animal_slices :&[&str] = &["Laksh","vikram","gargi"];
    println!("animal slices are {:?}",animal_slices);
    let book_slices: &[&String] = &[&"IT".to_string(),&"harry potter".to_string(),&"ZEN".to_string()];
    println!("book slices are {:?}",book_slices);


    // String VS String slices (&str)
    // Strings [growables, mutable,owned string type]
    let  mut stone_cold : String = String::from("hell, ");
    stone_cold.push_str("Yeah!");
    println!("stone Cold Says: {}",stone_cold); 

    // &str
    let phase : String = String::from("Hello , world");
    let s :&str = &phase;
    println!("the phase is {:?}",s);
}