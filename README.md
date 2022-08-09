# chrome_password

~~Steal~~ Get username & password from Chrome. (Now Only Windows)

## Installation

You can download the exe binary from the
[releases page](https://github.com/justjavac/chrome_password.rs/releases).

**With Cargo:**

```powershell
cargo install chrome_password
```

## Usage

```plain
➜  ~  chrome_password
+----------------------------------------+------------------------+---------------------------+
| url                                    | username               | password                  |
+----------------------------------------+------------------------+---------------------------+ 
| https://baidu.com                      | justjavac              | 12345678                  | 
+----------------------------------------+------------------------+---------------------------+ 
| http://127.0.0.1/login.php             | admin                  | 12345678                  | 
+----------------------------------------+------------------------+---------------------------+ 
| https://www.zhihu.com/login            | root                   | 12345678                  |
```

## Use as crate

Add this to your `Cargo.toml`:

```toml
[dependencies]
chrome_password = "0.1"
```

Code:

```rust
use std::env;
use std::path::PathBuf;

fn main() {
  let user_profile = env::var("LOCALAPPDATA").unwrap();
  let local_state_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Local State");
  let login_data_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Default/Login Data");

  let master_key = chrome_password::get_master_key(&local_state_path);
  let password = chrome_password::get_password(&login_data_path, &master_key);

  println!("{:?}", &password);
}
```

## License

Deno Version Manager(dvm) is released under the MIT License. See the bundled
[LICENSE](./LICENSE) file for details.
