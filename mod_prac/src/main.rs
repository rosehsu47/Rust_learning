//要指名用哪個文件夾裡面的東西
//對於每個文件夾裡面的東西都預設是private, 要設成pub
//需要公開的屬性也要設pub
//mod一般寫在main裡面

mod app;

// use app::models::Dog;
// use app::models::Animal;
use app::models::*;
use app::db;

fn main() {
    println!("Hello, world!");
    
    let peter = Dog {name: String::from("Peter"), age: 7 };
    peter.run();
    db::save_dog(&peter);
    peter.sleep();
    peter.eat();

    println!("{:?}", peter );
}

