// use std::io::{self};

// use std::vec;

// use std::collections::HashMap;

// use std::{fmt::{Debug, Display}};

mod error;

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
    // Vectors allow us to store more than one value in a single data structure that puts all the values next to
    // each other in memory and can only store value of the same type;

    // let mut v:Vec<i128>=Vec::new();
    // // using macro
    // // let v=vec![2,3,4];
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // v.push(59);
    // v.push(9);

    // reading elements of vectors
    // let eight=v[8];  // it will throw an error
    // print!("{}",eight);

    // use option enum
    // let third:Option<&i128>=v.get(5);
    // match third {
    //     Some(third)=>println!("The third element is {third}"),
    //     None=>println!("There is no third element")
    // }
    // // using for loop
    // for i in  &mut v{
    //     *i*=50;
    //     println!("{i}")
    // }

    //use enum to hold different types of value but as a single type in vector, if we know the types of value then we use vectors otherwise we go for traits
    // let v=vec![
    //     Student::name(String::from("Subhash")),
    //     Student::roll_number(2),
    // ];
    // let first=&v[0];

    //.........................................................................
    // HashMap
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // // let team_name=String::from("Blue");
    // // let score=scores.get(&team_name).copied().unwrap_or(0);

    // // overwriting a Value
    // scores.insert(String::from("Blue"), 30);

    // // Adding a key and value only if a key isn't present
    // scores.entry(String::from("Red")).or_insert(19);
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }
    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{map:?}")
    // .................................................................

    //  Generics:- Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication;

    // let number_list = vec![1, 2, 34, 5, 6, 7];
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is: {largest}");
    // // what if i have 100 list and i want to find largest number in every list, we have to implement this for loop 100 times
    // // instead we can make a generic function that will do it for every list
    // fn largest(list: &[i32]) -> &i32 {
    //     let mut largest = &list[0];

    //     for number in list {
    //         if number > largest {
    //             largest = number;
    //         }
    //     }
    //     largest
    // }
// generic function
// fn largest<T:std::cmp::PartialOrd>(list:&[T])->&T{
//   let mut largest=&list[0];
//   for item in list{
//     if largest<item{
//         largest=item
//     }
//   }
//   largest
// }

// generic struct
// let integer=Point{x:5,y:7};
// let float=Point{x:2.0,y:4.9};
// let double=Pointer{x:3,y:9.8};

// generic enum
// enum Option<T>{
//     Some(T),
//     None,
// }

//In method definition
// let p=Point{x:3,y:4};
// println!("{}",p.x())



//  Traits:- A trait defines functionality a particular type has and can share with other types.
// let rec1=Rectangle{
//     x:5,
//     y:6
// };
// notify(&rec1);
//print!("The area of the rectangle is {}",rec1.area());




//  Validating Refrences with Lifetimes
// let r;
// {
//     let x=6;
//     r=&x;
// }
// println!("r: {r}")

// let string1=String::from("abcd");
// let string2="xyz";
// let result=longest(string1.as_str(),string2);
// println!("The longest string is {result}");



//  errors handling
error::opening_file();
// handling error from the calling mod
let returned_error=error::read_data_from_file();
match returned_error {
    Ok(content)=>println!("The content is {}",content),
    Err(e)=>panic!("Error is {}",e)
}



}
// fn longest<'a>(x:& 'a str,y:& 'a str)->& 'a str{
//       if x.len()>y.len(){
//         x
//       }else{
//         y
//       }
// }


// implmenting traits
// pub struct Rectangle{
//     x:i32,
//     y:i32
// }
// impl CalculateArea for Rectangle {
//     fn area(&self)->i32 {
//         self.x*self.y
//     }
// }

// pub trait CalculateArea {
//     fn area(&self)->i32;
// }

// implementing trait as parameter
// pub fn notify(item:&impl CalculateArea){
//     println!("The item's area! {}",item.area());
// }

//  Trait bound syntax
// pub fn notify<T:CalculateArea>(item:&T){
//     println!("Inside the trait bound syntax, 
//     The area of the item is {}",item.area());
// }

// parameter is 2
// pub fn notify<T:CalculateArea> (item1:&T,item2:&T){
//    println!("In this the item1 and item2 must be of same type and implements tha traits of calculate area!");
// }
// pub fn notify_two(item1:&impl CalculateArea,item2:&impl CalculateArea){
//     println!("In this the itemq and item2 may be of different type but they must implement the trait Calculate area");
// }

//we can use + to specify differnet types of traits
// pub fn TooManyTraits(item1:&(impl CalculateArea,Display)){

// } 

// pub fn TooManyTraitsTwo<T:CalculateArea+Display>(items:&T){

// }


//  Using where clause
// fn some_function<T:Display+CalculateArea,U:Clone+Debug>(t:&T,u:&U){

// }

// instead of like this we can use where
// fn some_function_two<T,U>(t:&T,u:&U)->i32
// where T:Display+Clone,U:Clone+Display{
// 4
// }


// struct Point<T>{
//     x:T,
//     y:T
// }

// impl <T> Point<T> {
//     fn x(&self)->&T{
//         &self.x
//     }
// }
// struct Pointer<T,U>{
//     x:T,
//     y:U,
// }
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
//  enum for vector implementation
// enum Student{
//     name(String),
//     roll_number(i32),
// }