

fn main() {
//numbers in rust i=for signed intergers it can hold both negative and positive values,u=unsigned intergers it can only hold positive intergers
// some number types i8,i16,132,i64,i128,and if you want to make it unsigned just add replace the i with u; and for floating numbers replace the i with f
    let x:i32= -4;
    let _y:u8= 80;
    let _z:f32= 20.4;

    //boolean 
    let is_male = false;
    let _is_above_18 = false;

    // if you want your variable to be immutable just add the "mut keyword after the let keyword"
    let mut _p = "name";//example
    _p = "dominion";


    // if is_male {
    //     print!("is a male")
    // }else {
    //     print!("is not a male")
    // }

    // if is_male && _is_above_18 {
    //     print!("is a legal male")
    // }

    //rust string
    let greeting = String::from("hello world!");
    let char1 = greeting.chars().nth(0);

    //pattern matching
    match char1 {
        Some(c) => print!("\nc:{}",c),
        None => print!("No character found!")
    }

    // this ignores the spedified number existense
    // print!("c:{}",char1.unwrap());

    //conditionals and loop
     let sentence = String::from("this is the code");
     let first_word = get_first_word(sentence);
     print!("\nw:{}",first_word);

     for i in 0..10 {
        print!("\ni: {}",i)
     }

    // println!("{}",x);
    // print!("z: {}", _z);
    // println!("\nHello, world!");
    let a = 2;
    let b = 3;
    let sum = do_sum(a, b);
    print!("this is the sum:{}",sum)
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