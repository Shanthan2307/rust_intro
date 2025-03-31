use webbrowser;

fn main() {
    println!("Hello, world!");
    let link:String = String::from("https://www.youtube.com/watch?v=KOaeDHeJ80I");
    meme(&link);
    let name = "Laksh";
    name_bad(name);
    dev_fun("Gargi",20,"web dev");
    let dev_exp = {
        let name = "Laksh";
        let age:i32 = 18;
        let dev = "dev lead";
        println!("hey I'm {},and age is {}.I working as {}",name,age,dev);
    };
    let x = {
        let price :i32 = 5;
        let qty :i32 = 10;
        price*qty
    };
    println!{"{}",x};

    //Return statement is not needed you can just use an expression which is technically returns a value so doesn't need to end with a semicolon
    let a:i32 = 87678;
    let b:i32 = 098889;
    let c : i32 = add(a,b);
    println!("the sum is {}",c);
}   

fn add(a:i32,b:i32)->i32{
    a+b
}

fn dev_fun(name:&str,age:i32,dev:&str){
    println!("hey I'm {},and age is {}.I working as {}",name,age,dev);
}

fn name_bad(name:&str){
    println!("my name is {}, and if you are bad , I'm your dad",name);
}



fn meme(link:&str){
    if webbrowser::open(link).is_ok(){
        println!("start the learning again");
    }
    else{
        println!("failed , check  the logic");
    }
}
