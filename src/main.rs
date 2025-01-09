

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
    let mut  s2 = &mut s1;
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