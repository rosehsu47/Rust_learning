use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();

    let x = format!("{},{}","Rose","are u ok");
    println!("{}",x);

    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let mut ten_zeros: [i64; 10] = [0; 10];
    println!("{:?}", middle);

    let tuple: (i32, &str) = (50, "hello");
    let (_ , word ) = tuple;
    println!("{},{}","", word);

    let mut a = add1(8);
    println!("{}",a);

    a = add2(a) ;
    println!("{}",a);

    let t1 = (1, 2);
    let t2 = (11, 22);
    let t3 = (1, 2);
    let t4 = ("1", 2);
    
    println!("{}", t1 == t2);
    println!("{}", t1 == t3);
    //println!("{}", t1 != t4);

    let array = [a;3];
    println!("{:?}", array);

    let mut vec1: Vec<&str> =  vec!["aa","dd","qq"];
    println!("{:?}", vec1);
    vec1.push("++");
    println!("{:?}", vec1);

    let through_away = vec1.pop();
    println!("{:?}", through_away);
    println!("{:?},{},{:?}", vec1,"w",through_away);

    let c = through_away.unwrap();
    println!("{}", c);

//     let number = Some(7);
// let mut optional = Some(0);

// // If `let` destructures `number` into `Some(i)`, evaluate the block.
// if let Some(i) = number {
//     println!("Matched {:?}!", i);
// } else {
//     println!("Didn't match a number!");
// }

// // While `let` destructures `optional` into `Some(i)`, evaluate the block.
// while let Some(i) = optional {
//     if i > 9 {
//         println!("Greater than 9, quit!");
//         optional = None;
//     } else {
//         println!("`i` is `{:?}`. Try again.", i);
//         optional = Some(i + 1);
//     }
// }

}
fn add1(i: i8) -> i8 {
    return i + 1 ;
}
fn add2(i: i8) -> i8 { i + 2 }

