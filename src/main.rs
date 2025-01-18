

mod hashmapstruct;
use std::{collections::{btree_map::{Keys, Values}, HashMap}, usize};

use hashmapstruct::HashMapStruct;

fn group_values_by_key(pairs: Vec<(String,i32)>) -> HashMap<String,i32> {
  let mut new_hashmap = HashMap::new();
  for (key ,val) in pairs {
   new_hashmap.insert(key, val);
  }
  return new_hashmap;
}

fn main() {
// let vec = vec![1,2,3,4];
// let mut v_iter = vec.iter();
let mut score_map = HashMap::new();
// score_map.insert(String::from("dominion"), 50);
// score_map.entry(String::from("gina")).or_insert(49);
// print!("{:?}",score_map);

let text = "Hello world Wonderful world";

for word in text.split_whitespace() {
let count = score_map.entry(word).or_insert(0);
*count += 1;
}

}

// fn even_filter(v: &mut Vec<i32>) {
//    let mut i = 0;
//    while i < v.len() {
//     if v[i] % 2 != 0 {
//       v.remove(i);
//     }else {
//          i += 1;
//     }
//    }
// }

// fn even_value (v: &Vec<i32>) -> Vec<&i32> {
//   let mut new_vec = Vec::new();
//   for val in v  {
//       if val % 2 == 0 {
//         new_vec.push(val);
//       }
//   }
//   return new_vec;
// }

// fn string_slice(s: &mut String) -> &str {
//   let byte = s.as_bytes();
//   let mut slice = "";
//   for (i,&items) in byte.iter().enumerate() {
//     let c = items as char;
//     if c == ' ' {
//         break;
//     }
//      slice = &s[0..i+1];
//   }
//   return slice;
// }


// fn read_file_content(path: &str) {
// let res = fs::read_to_string(path);
// match res {
//     Ok(content) => println!("{}",content),
//     Err(_) => println!("error occurred"),
// }
// }

//numbers in rust i=for signed intergers it can hold both negative and positive values,u=unsigned intergers it can only hold positive intergers
// some number types i8,i16,132,i64,i128,and if you want to make it unsigned just add replace the i with u; and for floating numbers replace the i with f
    // let x:i32= -4;
    // let _y:u8= 80;
    // let _z:f32= 20.4;

    //boolean 
    // let is_male = false;
    // let _is_above_18 = false;

    // if you want your variable to be immutable just add the "mut keyword after the let keyword"
    // let mut _p = "name";//example
    // _p = "dominion";


    // if is_male {
    //     print!("is a male")
    // }else {
    //     print!("is not a male")
    // }

    // if is_male && _is_above_18 {
    //     print!("is a legal male")
    // }

    //rust string
    // let greeting = String::from("hello world!");
    // let char1 = greeting.chars().nth(0);

    //pattern matching
    // match char1 {
    //     Some(c) => print!("\nc:{}",c),
    //     None => print!("No character found!")
    // }

    // this ignores the spedified number existense
    // print!("c:{}",char1.unwrap());

    //conditionals and loop
    //  let sentence = String::from("this is the code");
    //  let first_word = get_first_word(sentence);
    //  print!("\nw:{}",first_word);

    //  for i in 0..10 {
    //     print!("\ni: {}",i)
    //  }

    // // println!("{}",x);
    // print!("z: {}", _z);
    // println!("\nHello, world!");
    // let a = 2;
    // let b = 3;
    // let sum = do_sum(a, b);
    // print!("this is the sum:{}",sum)

    // let r = "Alice";
    // let mut x = r.to_string();
    // x.push_str(",Hello");

    // let y = x.as_str();

    // print!("{}",y)
    // heap_fn();

    //ownership
    // let mut s1 = String::from("Hello");

    //having more than one mutable reference
    // let mut  s2 = &mut s1;
    // s2.push_str(" weirdo");
    // let s3 = &mut s2;
    // s3.push_str("okay!");
    // println!("s3: {}",s3)
    // print!("{}",s1);
    // let s2 = s1;
    // println!("\n{}",s2);
    // let _s2 = take_owner(s1.clone());
    // print!("{}",_s2)
    // let s2 = String::from("world!");

    // let fmt = format!("{} {}",s1,s2);
    // print!("{}",fmt)
    // take_owner(&s1);
    // print!("{}",s1); 
    // s1 = take_owner(s1);
    // print!("{}",s1);
  //   // run_struct();
  // let updated =   update_word(&mut s1);
  // print!("updated string: {}",updated)
// let user1 = User {
//   username: String::from("dominion"),
//   is_active: true,
//   signin_count: 30
// };
// user1.username = String::from("Alice");

// print!("username: {:?}", user1.username)
// let rect = Rect {
//   width: 32,
//   height: 4
// };
// let area = rect.area();

// print!(
// "{:?}", area
// )

// let circle = Shapes::Circle(28.3);
// let rectangle = Shapes::Rectacle(23.4, 6.0);
// let square = Shapes::Square(5.5);

// print_shape_value(rectangle);

// let ans = calculate_area(circle);
// print!("{}",ans)
// enum Result<T,R> {
//   Ok(T),
//   Err(R)
// }

// let ans = read_file_content(String::from("example.txt"));  
// let mut input = String::new();
// io::stdin().read_line(&mut input).expect("something went wrong");

// let input:u32 =match input.trim().parse() {
//   Ok(num) => num,
//  Err(_) => 0
//  };

// let car = Car {
//   model: String::from("Camry"),
//   color: String::from("blue"),
//   price: 4000.78
// };
// let x = Car::new("Camry".to_string(),"black".to_string(),4000.56);
// let camry = CarModel::Camry("camry".to_string(), "black".to_string(), 40.4);
// let x = get_car(camry);
// print!("{}",x)
// let maybe_num = Some(42);
// let Some(value) = maybe_num else {
//   print!("value cannot be empty");
//   return;
// };

// let full_name:&String = &String::from("Dominion Isicheri");
// let first_char = full_name.chars().nth(0);

// match first_char {
//     Some(e) => print!("{}",e),
//     None => println!("this string is empty")
// }

// let count = count_characters(full_name);
// print!("{}",full_name);
// print!("{}",count);
// let num = get_firsta(String::from("main"));
//   match num {
//     Some(value) => print!("{}",value),
//     None => print!("value does not exist"),
//   }

// let file = fs::read_to_string("./path.txt");

// match file {
//     Ok(value) => print!("{}",value),
//     Err(_) => print!("file does not exist"),
// }

// }

// fn get_firsta(s: String) -> Option<i32> {
// for (index,char) in s.chars().enumerate() {
//   if char == 'a' {
//   return  Some(index.try_into().unwrap());
//   }
// }
// return  None;
// }

// fn count_characters(str:&String) -> usize {
// str.chars().count()
// }

// fn get_first_word(s: String)-> String {
// let c = s.chars();
// let mut  b = String::new();
// for x in c  {
//   b.push_str(x.to_string().as_str());
//   if x == ' ' {
//     break;
//   }
// }
// return b;
// }

// fn get_car(car:CarModel) -> CarModel {
//   match car {
//       CarModel::Camry(x,y ,z ) => CarModel::Camry(x, y, z),
//           CarModel::Bmw(x, y, z) => CarModel::Bmw(x, y, z),
//           CarModel::Benx(x, y, z) => CarModel::Benx(x, y, z),
//   }
// }

// fn print_car(car:CarModel) {
//   if let CarModel::Camry(x,y ,z ) = car {
//     print!("{}{}{}", x,y,z);
//   }
// }

// enum CarModel {
//     Camry(String,String,f64),
//     Bmw(String,String,f64),
//     Benx(String,String,f64)
// }

// struct Car {
//   model: String,
//   color: String,
//   price: f64
// }

// impl Car {
//     fn new(x:String,y:String,z:f64) -> Self {
//      Self { model: x, color: y, price: z }
//     }
// }


// fn read_file_content(path: String) -> Result<String, String> {
//   let res = fs::read_to_string(path);

//   // lets try this error handling in another way
//   // let x = match res {
//   //     Ok(content) => Ok(content),
//   //     Err(err) => Err(err.to_string())
//   // };
//   if let Result::Ok(content) = res {
//     return Ok(content);
//   }else {
//        return Err("An Error occured".to_string());
//   }


//   // rust doesnt have a concet of a null type

// }

// //unit struct is a struct that does not have a datatype associated with it
// struct Car;

// enum Shapes {
//     Circle(f64),
//     Square(f64),
//     Rectacle(f64,f64)
// }

// fn calculate_area(shape:Shapes) -> f64{
// let ans = match shape {
//  Shapes::Circle(radius) => std::f64::consts::PI * radius*radius,
//  Shapes::Square(length) => length * length,
//  Shapes::Rectacle(height, width) => height * width
// };

//   return ans;
// }

// fn print_shape_value(shape:Shapes)  {

//   if let Shapes::Circle(radius) = shape {
//     print!("circle radius: {}",radius)
//   }else if let Shapes::Square(length) = shape {
//     print!("square length: {}",length);
//   }else if let Shapes::Rectacle(heigth, width) = shape {
//     print!(" lenght and width of the rectacngle{} {}", heigth,width);
//   }

// }

// enum Directions {
//   North,
//   South,
//   East,
//   West  
// }

// fn get_Direction(direction: Directions) -> Directions {
//   return direction;
// }

// struct User {
//   username: String,
//   is_active: bool,
//   signin_count: i64
// }

// struct Rect {
//   width: i32,
//   height: i32
// }

// impl Rect {
//     fn area(&self) -> i32 {
//     self.width * self.height
//     }
//     fn new(width:i32,height:i32) -> Self{
//       Self {width,height}
//     }
// }

// fn heap_fn() {
//     let mut a = String::from("Hello");
//     a.push_str("world!");
//    println!("{},{},{:p}",a.len(),a.capacity(),a.as_ptr());
//   for i in 0..10 {
//     a.push_str("world!");
//     println!("{},{},{:p}",a.len(),a.capacity(),a.as_ptr());
//   }
// }


// fn run_struct() {

//   struct Point {
//     x:i32,
//     y:i32
//   }

//   impl Point {
//     fn new(x:i32,y:i32) -> Self {
//     Self { x: (x), y: (y) }
//     }
//   }
// }


// fn opt_fn(opt: Option<i32>) -> Option<i32>{
//     return opt.map(|x| x * x)
// }

// fn get_first_word(sentence: String) -> String{
//  let mut ans = String::new();
// for char in sentence.chars() {
//     ans.push_str(char.to_string().as_str());
//     if char == ' ' {
//         break;
//     }
// }
// return ans;
// }

// fn do_sum(a:i32,b:i32) -> i32{
//   return a + b;
// }

// fn take_owner(s: String)-> String {
// return s
// }

// fn update_word(word: &mut String) -> String {
//   word.push_str(" world");
//   return word.to_string();
// }