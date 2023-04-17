//Different traits to impact the way T is interpreted

//inputs with Display trait
fn info<T: std::fmt::Display>(a: &T){
    println!("{}", a);
}

//inputs with ToString method
fn info2<T: ToString>(a: &T){
    println!("{}", a.to_string());
}

//inputs with string slice trait
//when taking reference to input, is it a UTF-8 set of bytes?
fn info3<T: AsRef<str>>(a: &T){
    println!("{}", a.as_ref());
}


fn main(){
    let s: String = "Hi".to_string();
    let s2: &str = "hi";

    info3(&s);
    info3(&s2);
}
