fn main() {
    println!("Hello, world!");
    let a:u32 = sum(5,10);
    println!("the requried sum is {}",a);
}

fn sum(a:u32,b:u32)->u32{
    return a+b;
}