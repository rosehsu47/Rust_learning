fn main() {
   
    println!("Hello, world!");
    let array = [1, 2, 3, 4, 5];
    print_array(&array);
    
    let vec= vec![1,2,3,4,5];
    
    let s = sum(&vec);
    println!("{}", s);

    let t = 5;
    let s2 = sum_set_init(&vec, t);
    println!("{}", s2);

    let peter = Dog{
        name : "Peter".to_string(),
        age : 7,
    };

    peter.run();

    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    peter.sleep();
    peter.eat();

    let direction = Direction::East ;

    match direction {
    Direction::East => println!("{}","East" ),
    Direction::West => println!("{}","West" ),
    _ => println!("{}","nothing" ),
    }

    if let Direction::West = direction {
        println!("{}", "NOPE");
    }else {
        println!("{}","I am not West." );
    }

    let is_some = Myoption::Some("要求一定要放string進來".to_string());
    let is_none = Myoption::None;

    if let Myoption::Some(String) = &is_none {
        println!("{}","恭喜盒子裡有東西");
    } else {
        println!("{}","None" );
    }
    
    match is_some{
        Myoption::Some(y) => println!("{}", y),
        Myoption::None => println!("{}","None" ),
    }

    println!("{}",&is_none.is_some());

    let myurl = make_url("localhost".to_string(),8080,"desktop".to_string());
    println!("{}",myurl );

    let my2ndurl = Url{
        host: "192.1.1.1".to_string(),
        port: 3000,
        path: "index".to_string(),
    };

    my2ndurl.url_to_string();
}


fn print_array(arr: &[i32;5]) {
    for i in arr {
        println!("{}", i);
    }    
}

fn sum(arr: &Vec<i32>) -> i32 {
     sum_set_init(arr,0)
}

fn sum_set_init(arr: &Vec<i32>, init: i32) -> i32 {

    let mut sum = init;
    for h in arr {
        sum += h;
    } 
    return sum;
}

struct Dog {

    name: String,
    age: i32,

}

impl Dog {
    fn run(self: &Self){
        println!("{} is running.", self.name );
    }
}

trait Animal {
    fn sleep(&self); 
    fn eat (self: &Self){
        println!("{}", "I want fooddddd!" );
    }
}

impl Animal for Dog {
    fn sleep(&self){
        println!("{}","zzZ" );
    }
}

enum Direction {
    East,
    West,
    North,
    South,
}

enum Myoption {
    Some(String),
    None
}

impl Myoption {
    fn is_some(&self) -> bool {
        if let Myoption::Some(_) = &self {
            true
        } else {
            false
        }

    }

}

fn make_url(host: String, port: u32, path: String) -> String {

    let url = format!("https://{0}:{1}/{2}", host, port, path);
    url

}

struct Url{
    host: String,
    port: u32,
    path: String,
}

impl Url{
    fn url_to_string(&self) -> String {
        let url = format!("https://{0}:{1}/{2}", self.host, self.port, self.path);
        println!("{},{}", "這是結構體裡的函數回傳的url",url);
        return url;
    }
}