```
$ git clone https://github.com/illicitonion/cargo-fix-2018-repro.git
$ cd cargo-fix-2018-repro
$ rustc +beta --version
rustc 1.31.0-beta.17 (1a4f1f398 2018-11-25)
$ cargo +beta --version
cargo 1.31.0-beta (339d9f9c8 2018-11-16)
$ cargo +beta fix --edition-idioms
    Checking rust-ignore-repro v0.0.1 (/home/dwh/tmp/rustplay)                                                                                                                                                                                 
warning: failed to automatically apply fixes suggested by rustc to crate `rust_ignore_repro`                                                                                                                                                   

after fixes were automatically applied the compiler reported errors within these files:

  * src/main.rs

This likely indicates a bug in either rustc or cargo itself,
and we would appreciate a bug report! You're likely to see 
a number of compiler warnings after this message which cargo
attempted to fix but failed. If you could open an issue at
https://github.com/rust-lang/cargo/issues
quoting the full output of this command we'd be very appreciative!

warning: `extern crate` is not idiomatic in the new edition                                                                                                                                                                                    
 --> src/main.rs:1:1                                                                                                                                                                                                                           
  |                                                                                                                                                                                                                                            
1 | extern crate ignore;                                                                                                                                                                                                                       
  | ^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`                                                                                                                                                                                           
  |                                                                                                                                                                                                                                            
  = note: `-W unused-extern-crates` implied by `-W rust-2018-idioms`                                                                                                                                                                           
                                                                                                                                                                                                                                               
warning: `extern crate` is not idiomatic in the new edition                                                                                                                                                                                    
 --> src/main.rs:1:1                                                                                                                                                                                                                           
  |                                                                                                                                                                                                                                            
1 | extern crate ignore;                                                                                                                                                                                                                       
  | ^^^^^^^^^^^^^^^^^^^^ help: convert it to a `use`                                                                                                                                                                                           
  |                                                                                                                                                                                                                                            
  = note: `-W unused-extern-crates` implied by `-W rust-2018-idioms`                                                                                                                                                                           
                                                                                                                                                                                                                                               
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
```
