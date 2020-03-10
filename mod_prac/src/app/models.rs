//想通過:?印出或轉換成字串時，需加上（在要印的東西之前）
#[derive(Debug)]
pub struct Dog{ 
    pub name: String ,
    pub age: i8 
}

impl Dog{
    pub fn run(&self) {
        println!("{} is running. ", &self.name);
    }
}

pub trait Animal {
    fn sleep(&self);    
    fn eat(&self) {
        println!("{}","yummy..." )
    }
}

impl Animal for Dog{
    fn sleep(&self){
        println!("{} : zZ", self.name );
    }
}
