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


// fn main(){
//     let name:String = String :: from("Raja");

//     println!("This is name : {}",name);

//     let mut name2 = name;
//     name2.push_str(" Kumar");
//     println!("this is name2 : {}", name2);

//     //println!("This is name : {}",name); this line gives error, because the owenership has been transferred to name2 , it will not give error if name.clone was written in line no.24
// }



// fn main(){
//     let name:String = String :: from("Raja");

//     let (len,name) = get_len(name);
//     println!("{}",len);
//     //println!("{}",name)//ownership has been transferred to s{
//     print!("name : {}",name);
// }

// fn get_len(s:String)->(usize,String){
//     println!("from the get_len function , s : {}",s);
//     return (s.len(),s);

// }




//BORROWING RULES
//You can only have one mutable references.If there ia a mutable reference , there can't be other immutable or mutable reference
//you  can have multiple immutable reference


// fn main(){
//     let mut name:String = String::from("Raja");

//     let str1: &mut String = &mut name;

//     println!("{}",str1);
//     //ownership of str1 ends here because str1 has not been used after this line, now you can make any number of immutatble reference
//     let str2: &String = &name;

//     //println!("{} {}",str2,str1);
//     println!("{}",str2);
//}




//Enum with value

use std::f32::consts::PI;

enum Shape{
    Square(f32),
    Circle(f32),
    Rectangle(f32,f32)
}


fn main(){
    println!("Here is the reuired answer : ");
    println!("Area of square : {}",calulate_area(Shape::Square(5.0)));
    println!("Area of square : {}",calulate_area(Shape::Circle(5.0)));
    println!("Area of square : {}",calulate_area(Shape::Rectangle(5.0,10.0)));


    println!("Here is the answer for perimeter");
    println!("Area of square : {}",calulate_perimeter(Shape::Square(5.0)));
    println!("Area of square : {}",calulate_perimeter(Shape::Circle(5.0)));
    println!("Area of square : {}",calulate_perimeter(Shape::Rectangle(5.0,10.0)));

}

fn calulate_area(s:Shape)->f32{

  match s{
        Shape::Square(a) => return a*a,
        Shape::Circle(a) => return PI*a*a,
        Shape::Rectangle(a, b) => return a * b
        
    }
}


fn calulate_perimeter(s:Shape)->f32{

    match s{
        Shape::Square(a) => return 4.0*a,
        Shape::Circle(a) => return 2.0*PI*a,
        Shape::Rectangle(a, b) => return 2.0*(a + b)
        
    }
}