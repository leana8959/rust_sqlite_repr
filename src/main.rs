use sqlite::Connection;

fn main() {
    let conn = Connection::open(":memory:").unwrap();

    conn.execute("CREATE TABLE users(name TEXT, age INTEGER, PRIMARY KEY (name))")
        .unwrap();

    let mut stmt = conn
        .prepare(
            "INSERT INTO users (name, age)
            VALUES('jean', 49)
            ON CONFLICT DO UPDATE SET age=49",
        )
        .unwrap();
    stmt.next().unwrap();

    let mut stmt = conn
        .prepare(
            "INSERT INTO users (name, age)
            VALUES('jean', 50)
            ON CONFLICT DO UPDATE SET age=50",
        )
        .unwrap();
    stmt.next().unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM users WHERE name='jean'")
        .unwrap();
    stmt.next().unwrap();

    let age = stmt.read::<i64, _>("age").unwrap();

    println!("Hello, world!");
    println!("age: {}", age);
}
