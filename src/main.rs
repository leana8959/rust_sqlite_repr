fn main() {}

#[test]
fn basic_test() {
    // Example code from SQLite
    // This would always work
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query).unwrap();

    let query = "SELECT * FROM users WHERE age > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, 50)).unwrap();

    statement.next().unwrap();
    assert_eq!("Bob", statement.read::<String, _>("name").unwrap());
    assert_eq!(69, statement.read::<i64, _>("age").unwrap());
}

#[test]
fn upsert_test() {
    use sqlite::Connection;

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

    println!("age = {}", age);
    assert_eq!(50, age);
}
