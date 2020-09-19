## rust_begginers_web
actix_web + diesel, diesel::r2d2, diesel::Sqlite

## how to use

```
cargo build
diesel migration run
cargo run
```

## request sample

insert title, body to posts table

```
curl -X POST -H "Content-Type: application/json" -d '{"title":"pool", "body":"xxx"}' 'localhost:8088/bar'
```

get titles

```
curl localhost:8088/foo 
```
