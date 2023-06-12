# Update
In the branch `compile_from_source`, I updated the Dockerfile to compile `sqlite3` from source. Doing so didn't solve the problem either.

## To reproduce the problem
Running
```sh
cargo test
```
is completely fine. However, on the otherhand, running
```sh
docker-compose build && docker-compose up
```
would fail.

The program would fail at this query:
```sql
INSERT INTO users (name, age)
VALUES('jean', 49)
ON CONFLICT DO UPDATE SET age=49
```
while giving this error message:
```text
message: Some("near "UPDATE": syntax error")
```

This query only fails when it's used in docker.
