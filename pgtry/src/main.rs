// extern crate postgres;

use postgres::{Client, NoTls};

fn main() {

    println!("{}","Start" );


    let db = "host=localhost user=rosehsu";


    // let mut conn =  Client::connect(db, NoTls)?;

    let mut conn = match Client::connect(db, NoTls) {

        Ok(conn) => {
            println!("{}","Connect successfully!" );
            conn
        },
        Err(e) => {
            println!("Connection error: {}", e);
            return;
        }
    };



    &conn.execute("create table if not exists blog (
        id serial primary key,
        title varchar(255),
        body text)", &[]).ok().expect("Table creation failed");

    let s_to_s = [1,2,3];
    change_type(s_to_s);
}

fn change_type (values: [i32 ; 3]) {

    let mut result : String = "".to_string(); 

    for num in &values {
        let num_to_str = num.to_string();
        &result.push_str(&num_to_str);
    }

    println!("{}",&result );
    
    // let result2 =  values as String;
    // result2
}


#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
 }
