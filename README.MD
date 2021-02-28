 # Rust Color
Rust Color is a terminal color rendering library,
thats supports 3/4 bit colors, 8 bit colors, 24 bit color
rendering output, compatible with windows.

Rust Color uses **ansi scaped sequences**. This [article](https://en.wikipedia.org/wiki/ANSI_escape_code) in
wikipedia explain how it works.

* How to install - Add the rustcolor crate to your Cargo.toml 
```
[dependencies]
rustcolor = {path = "../rustcolor"}
```