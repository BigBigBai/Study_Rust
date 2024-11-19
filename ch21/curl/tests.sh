cargo clean
cargo build
(cargo run -- "https://www.eecg.toronto.edu/~bli/ece1724/assignments/files/lab3.html"
cargo run -- "www.eecg.toronto.edu"
cargo run -- "data://www.eecg.toronto.edu"
cargo run -- "http//www.eecg.toronto.edu"
cargo run -- "https://[...1]"
cargo run -- "https://255.255.255.256"
cargo run -- "http://127.0.0.1:65536"
cargo run -- "https://example.rs"
cargo run -- "https://www.eecg.toronto.edu/~bli/ece1724/assignments/files/lab4.html"
cargo run -- "https://jsonplaceholder.typicode.com/posts" -d "userId=1&title=Hello World" -X POST
cargo run -- --json '{"title": "World", "userId": 5}' "https://dummyjson.com/posts/add"
cargo run -- --json '{"title": "World"; "userId": 5}' "https://dummyjson.com/posts/add"
)2>&1 | grep -v -E 'Compiling|Running|Finished' > tests.log
diff tests.log tests.expected