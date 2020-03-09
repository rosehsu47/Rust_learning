extern crate postgres;

use postgres::{Client, NoTls};

fn main() {

    println!("{}","Start" );


    let db = "host=localhost user=rosehsu";
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
}


