// use std::io::{self};

fn main() {
    // shadowing
    //Shadowing is different from marking a variable as mut because we’ll get a compile-time error
    //if we accidentally try to reassign to this variable without using the let keyword.
    // let x = 23;
    // let x = x + 2;
    // if true {
    //     let x = x * 3;
    //     print!("Inner Scope variable:{}\n", x);
    // }
    // print!("Outer Scope Variable: {}\n", x);

    // tuple can not increase size
    // let tup: (i128, i32, f32) = (20000, 23, 34.43);
    // we can access by indices
    // let c = tup.1;
    // destructuring
    // let (x, y, z) = tup;
    // print!("x: {}, y:{},z:{}, c: {}\n", x, y, z, c);

    // Array

    // let arr = [1, 2, 3];
    // println!("arr {}", arr[0]);
    // what will happen if we try to access the indice that is greater than the array length;
    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter the array index.");
    // let mut index = String::new();
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read index");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");
    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");

    // function:We define a function in Rust by entering fn followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.
    // another_function(2);
    // print_name('a', 'b');
    // let x=five();
    // let x=plus_one(x);
    // print!("The value of x:{x}");

    // loops

    // let condition=true;
    // let number =if condition {5} else{7};
    // println!("The number is {number}");

    // this will give the error
    // let number=if condition {8} else{"six"};

    //  generatic fibonacci number of 5
    // print_fibonacci(5);
    // for a in 1..5{
    //     let result:i32;
    //     if a!=1 && a!=2{
    //           result=a+a-1;
    //           println!("{result}")
    //     }else{println!("1")}
    // }

    // Function and ownership

    // let s1=do_something();
    // let s2=String::from(" is cool");
    // let s3=do_something_else(s2);
    // print!("{s1}");
    // print!("{s3}");

    //  refrencing and borrowing
    // let s="Ram";
    // let t=calculate_length(s);
    // print!("{t}");

    // let s=String::from("Rust Is Lust");
    // let t=calculate_length(&s);
    // println!("{t}");
    // print!("{s}");




    // ............................................................................
//  Structs in rust let you structure data together. Similar to objects in JavaScript

// let user1=User{
//     username:String::from("Subhash"),
//     email:String::from("Subhash@gmail.com"),
//     sign_in_count:3
// };

// let user2=User{
//    email:String::from("Jo"),
//     ..user1
// };

// print!("User 1 details: {:?} ,{:?},{:?}",user2.username,user2.email,user1.sign_in_count);
// let rect1=Rectangle{
//      width:23,
//      height:25
// };

// let rect2=Rectangle{
//     width:20,
//     height:20,
// };
// println!("The area of the rectangle
// with height and widht 25 and 23 respectively is
// {}",rect1.area(2));
// print!("{}",rect1.can_hold(&rect2));


//  enum.........................
// let four=IpAddrKind::V4(String::from("Four"));
// let six=IpAddrKind::V6(String::from("::1"));
// // we can also defind function on enum
// four.call();
// six.call();


// collection:-
// Vector:-.......






















}

// fn another_function(x:i16){
//     println!("Another function :{x}");
// }
// fn print_name(first:char,second:char){
//     println!("First and second are {first},{second}");
// }
// fn five()->i8{
//     6
// }
// fn plus_one(x:i8)->i8{
//     x+1
// }
// fn print_fibonacci(n: usize) {
//     let mut a = 0;
//     let mut b = 1;

//     for _ in 0..n {
//         println!("{}", a);
//         let next = a + b;
//         a = b;
//         b = next;
//     }
// }
// fn do_something()->String{
//     let x=String::from("Rust");
//     x
// }
// fn do_something_else(y:String)->String{
//     let z=String::from(y);
//     z
// }

// fn calculate_length(s:&String)->usize{   // s is a refrence to a string
//     s.len()
// }  // Here, s goes out of the scope. But because it does not have ownership of what
// // it rferes to, it is not dropped;

// fn try_to_change(s:&String){
//    s.push_str("Not bad");
// }

// .............................................................................
//  Slice  :  write a function that takes a string of words separated by spaces and 
// returns the first word it finds in that string. If the function doesn’t find a 
// space in the string, the whole string must be one word, so the entire string should be returned.

// fn first_word(s:&String)->&str{
//     let bytes=s.as_bytes(); // making a string to array of bytes
//     for(i,&item) in bytes.iter().enumerate(){
//         if item==b' ' {
//             return &s[0..i]
//         }
//     }
//     &s[..]
// }


//  struct
// struct User{
//     username:String,
//     email:String,
//     sign_in_count:u64,
// }

// struct Rectangle{
//     width:u32,
//     height:u32,
// }
// impl Rectangle {
//     fn area(&self,x:u32)->u32{
//         self.height*self.width*x
//     }
//     fn can_hold(&self,other:&Rectangle)->bool{
//         self.width>other.width&&self.height>other.height
//     }
// }

// Enum

// enum IpAddrKind{
//     V4(String),
//     V6(String)
// }
// impl IpAddrKind {
//     fn call(&self){
          
//     }
// }





















