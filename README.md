# proxy-server

Low level proxy server.
To implement request proxying, only standard `TcpStream` was used without additional libraries.

## Examples

With default params:

```rust
use proxy_server::Builder;

fn main() {
	Builder::new().bind().expect("Error in proxy");
}
```

With custom params:

```rust
use proxy_server::{log::LogLevel, Builder};

fn main() {
	Builder::new()
		.with_address("127.0.0.1:3000")
		.with_target("127.0.0.1:3001")
		.with_log_level(LogLevel::Warn)
		.with_threads(4)
		.bind()
		.expect("Error in proxy");
}
```
 With check and change target if needed on ev  ery request                                       
 ```rust    
fn get_actual_target(old: &str) -> &'static str {
	let target1 = "127.0.0.1:3001";
	let target2 = "127.0.0.1:3003";
	let res = match old {
		"127.0.0.1:3001" => target2,
		"127.0.0.1::3003" => target1,
		_ => target1,
	};
	res
}
  
fn main() {
	let cb: ChangeTarget = |old| get_actual_target(old);
	Builder::new()
		.bind(Some(cb))
        	.expect("Error in proxy");
}
```
