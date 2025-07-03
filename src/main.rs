// fn main() {
//     println!("Hello, world!");
//     let a:u32 = sum(5,10);
//     println!("the requried sum is {}",a);
// }

// fn sum(a:u32,b:u32)->u32{
//     return a+b;
// }



//Ownership rule in Rust
//Each value in Rust has an owner
//There can be only one owner at a time
//When the owner goes out of scope , the value will be dropped


fn main(){
    let name:String = String :: from("Raja");

    println!("This is name : {}",name);

    let mut name2 = name;
    name2.push_str(" Kumar");
    println!("this is name2 : {}", name2);

    //println!("This is name : {}",name); this line gives error, because the owenership has been transferred to name2 , it will not give error if name.clone was written in line no.24
}