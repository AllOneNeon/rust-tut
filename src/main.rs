
fn main() {
    let list = vec![1, 2, 3, 4, 5];
    list[100];
}

// fn main() {
//     let str = String::from("TEXT");
//     print_val(str.);
//     println!("{}", str);

//     let num = 9;

// }

// fn print_val(s: String) {
//     println!("{}", s);
// }

// fn print_num() {
    
// }

// fn main() {
//     let tuple = (12, 34.6, String::from("LOL"));
//     println!("{:?}", tuple);
// }

// fn main() {

//     const NUM: i8 = 3;
//     println!("{}", NUM);

//     // let arr = [2; 50];
//     // println!("{:?}", arr);
// //     let mut array: [i8; 5] = [1, 2, 3, 4, 5];
// //     array[2] = 10;
// //     println!("{:?}", array[2]);
// //     println!("{:?}", array);
// }

// use std::io;
// fn main() {
//     // i - input, o - output
//     let mut name : String = String::new();
//     println!("Input something: ");
//     match io::stdin().read_line(&mut name) {
//         Ok(_) => {
//             println!("Hi, {}", name);
//         },
//         Err(e) => {
//             println!("ERROR! - {}", e);
//         }
//     }
// }

// fn main() {
//     let numx = 3;
//     let num = match numx {
//         2 => 1, 
//         3 => 10,
//         // 3..=10 => 7,
//         _ => 0
//     };
//     println!("{}", num);
// }

// fn main() {
//     let num = 24;
//     match num {
//         10 => println!("Num is 10"),
//         23 => {
//             println!("Num is 23");
//             println!("Num is matched");
//         },
//         10..= 50 => {
//             println!("Num between 10 and 50");
//         }
//         _ => {
//             println!("No matches");
//             println!("try again");            
//         }
//     }
// }

// fn main() {
//     //loop
//     //while
//     //for
//     let mut num = 0;

//     for i in 0..101 {
//         println!("{}", i);
//     }

//     // while num <= 100 {
//     //     println!("{}", num);
//     //     num += 2;
//     // }

//     // loop {
//     //     println!("{}", num);
//     //     num += 1;
//     //     if num == 100 {
//     //         break;
//     //     }
//     // }
// }

// fn main() {
//     let is_true: bool= false;
//     let num = if is_true {
//         1
//     } else {
//         0
//     };
//     println!("{}", num);
// }

// fn main() {
//     let name: String = String::from("Kate");
//     if name != "Jordan" {
//         println!("Run");
//     } else if name == "Jordan"{
//         println!("hi jordan");
//     }
// }

// fn main(){
//     // or = || and - $$
//     let num = 25;
//     if num > 10 && num < 50 {
//         println!("cool!");
//     } 
// }

// fn main() {
//     //char
//     //bool
//     let logic: bool = true;
//     let logic1: bool = false;
//     println!("{}, {}", logic, logic1);
//     // let symbol = 'S';
//     // let text = String::from("Some text..");
//     // println!("{}", text);
//     // println!("{}", symbol);
// }

// fn main() {
//     let mut name = String::from("Andrew");
//     println!("My name is {}", name);
// }

// integer, float
// i8, i16, i32, i64, i128, isize(такое количесво бит сколько разрадная система(64))
// -127 -> 128, ...
// mutable
// u8, u16, u32, u64, u128, usize
//
