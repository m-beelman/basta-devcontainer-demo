extern crate postgres;
use std::env;

use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let username = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let hostname = env::var("POSTGRES_HOSTNAME").expect("POSTGRES_HOSTNAME must be set");
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB_NAME must be set");

    let mut client = Client::connect(
        &format!("postgresql://{}:{}@{}/{}", username, password, hostname, db_name),
        NoTls,
    )?;
    //Ok(());
    
    client.batch_execute(
        "
        DROP TABLE IF EXISTS app_user
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE app_user (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    )?;

  
    client.batch_execute(
        "
    INSERT INTO app_user (username, password, email) VALUES
    ('ada_lovelace', 'password123', 'ada_lovelace@example.com'),
    ('alan_turing', 'password123', 'alan_turing@example.com'),
    ('grace_hopper', 'password123', 'grace_hopper@example.com'),
    ('linus_torvalds', 'password123', 'linus_torvalds@example.com'),
    ('bill_gates', 'password123', 'bill_gates@example.com'),
    ('steve_jobs', 'password123', 'steve_jobs@example.com'),
    ('steve_wozniak', 'password123', 'steve_wozniak@example.com'),
    ('tim_berners_lee', 'password123', 'tim_berners_lee@example.com'),
    ('richard_stallman', 'password123', 'richard_stallman@example.com'),
    ('dennis_ritchie', 'password123', 'dennis_ritchie@example.com');
    ",
    )?;

    // request data and print it
    for row in client.query("SELECT id, username, email FROM app_user", &[])? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
//        let email: &str = row.get(2);
        println!("found person: {} {}", id, username);
    }

    Ok(())
}