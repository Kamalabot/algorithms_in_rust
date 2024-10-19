use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use std::thread;

fn fetch_data(client_id: usize, filter_value: usize, connection: Arc<Mutex<Connection>>) {
    let loc_con = connection.lock().unwrap();

    let mut stmt = loc_con
        .prepare("SELECT * FROM users WHERE age > ?1")
        .unwrap();

    let rows = stmt
        .query_map([filter_value as i32], |row| {
            Ok((row.get::<_, String>(1)?, row.get::<_, i32>(2)?))
        })
        .unwrap();

    let count = rows.count();
    println!("Client {client_id} fetched {count} records where age is > {filter_value}")
}

fn create_sample_db() -> Result<()> {
    let conn = Connection::open("thread_rust.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        age INTEGER NOT NULL
        )",
        [],
    )?;
    let mut stmt = conn.prepare("INSERT INTO users (name, age) VALUES (?1, ?2) ")?;
    for idx in 0..100 {
        stmt.execute([&format!("User-{}", idx), &format!("{}", idx % 50 + 20)])?;
    }
    Ok(())
}
fn main() -> Result<()> {
    println!("Working on database...");
    create_sample_db()?;
    let conn = Connection::open("thread_rust.db")?;
    // above conn instance is made into a thread safe connection
    let shared_conn = Arc::new(Mutex::new(conn));
    let mut handles = vec![];
    // Spawn 100 threads simulating 100 clients
    for client_id in 1..=100 {
        // then shared_conn is cloned here
        let conn_clone = Arc::clone(&shared_conn);
        let filter_value = client_id % 50; // Each client uses a different filter value

        let handle = thread::spawn(move || {
            fetch_data(client_id, filter_value, conn_clone);
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap()
    }
    println!("All clients completed fetching");
    Ok(())
}
