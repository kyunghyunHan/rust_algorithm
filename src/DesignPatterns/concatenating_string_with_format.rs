fn say_hello(name:&str)->String {
    
   name.to_string().push_str("dd");
   name.to_string()
}

pub fn main(){
    say_hello("dds");
    println!("{}",say_hello("dd"))
}