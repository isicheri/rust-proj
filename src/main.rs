use std::fs;



fn main() {
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
    let mut s1 = String::from("Hello");

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



}

fn read_file_content(path: String) -> Result<String, String> {
  let res = fs::read_to_string(path);

  // lets try this error handling in another way
  // let x = match res {
  //     Ok(content) => Ok(content),
  //     Err(err) => Err(err.to_string())
  // };
  if let Result::Ok(content) = res {
    return Ok(content);
  }else {
       return Err("An Error occured".to_string());
  }


  // rust doesnt have a concet of a null type

}

//unit struct is a struct that does not have a datatype associated with it
struct Car;

enum Shapes {
    Circle(f64),
    Square(f64),
    Rectacle(f64,f64)
}

fn calculate_area(shape:Shapes) -> f64{
let ans = match shape {
 Shapes::Circle(radius) => std::f64::consts::PI * radius*radius,
 Shapes::Square(length) => length * length,
 Shapes::Rectacle(height, width) => height * width
};

  return ans;
}

fn print_shape_value(shape:Shapes)  {

  if let Shapes::Circle(radius) = shape {
    print!("circle radius: {}",radius)
  }else if let Shapes::Square(length) = shape {
    print!("square length: {}",length);
  }else if let Shapes::Rectacle(heigth, width) = shape {
    print!(" lenght and width of the rectacngle{} {}", heigth,width);
  }

}

enum Directions {
  North,
  South,
  East,
  West  
}

fn get_Direction(direction: Directions) -> Directions {
  return direction;
}

struct User {
  username: String,
  is_active: bool,
  signin_count: i64
}

struct Rect {
  width: i32,
  height: i32
}

impl Rect {
    fn area(&self) -> i32 {
    self.width * self.height
    }
}

fn heap_fn() {
    let mut a = String::from("Hello");
    a.push_str("world!");
   println!("{},{},{:p}",a.len(),a.capacity(),a.as_ptr());
  for i in 0..10 {
    a.push_str("world!");
    println!("{},{},{:p}",a.len(),a.capacity(),a.as_ptr());
  }
}


fn run_struct() {

  struct Point {
    x:i32,
    y:i32
  }

  impl Point {
    fn new(x:i32,y:i32) -> Self {
    Self { x: (x), y: (y) }
    }
  }
}


fn opt_fn(opt: Option<i32>) -> Option<i32>{
    return opt.map(|x| x * x)
}

fn get_first_word(sentence: String) -> String{
 let mut ans = String::new();
for char in sentence.chars() {
    ans.push_str(char.to_string().as_str());
    if char == ' ' {
        break;
    }
}
return ans;
}

fn do_sum(a:i32,b:i32) -> i32{
  return a + b;
}

fn take_owner(s: String)-> String {
return s
}

fn update_word(word: &mut String) -> String {
  word.push_str(" world");
  return word.to_string();
}