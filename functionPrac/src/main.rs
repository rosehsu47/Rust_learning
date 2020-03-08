fn main() {
    println!("Hello, world!");
  
    let time : Time = Time::Morning ; 
    let mut aisatsu : String ;

    match time {
        Time::Morning => aisatsu = "Good morining".to_string(),
        Time::Afternoon => aisatsu = "Good afternoon".to_string(),
        _ => aisatsu = "GO TO SLEEP".to_string(),
    }

    let a_string = format_string( aisatsu ,"Rose".to_string() , "Hsu".to_string()) ; 

    println!("{0}! How are you?", a_string);

} 

fn format_string (aisatsu: String, a: String, b: String) -> String { 
    return format!("{} ,{} {}", aisatsu ,a, b)
}

enum Time {
    Morning,
    Afternoon,
    Evening,
    Night,
}
