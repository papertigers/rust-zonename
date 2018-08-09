rust-zonename
=============

Rust bindings to [getzoneid(3C)](https://illumos.org/man/3C/getzoneid),
[getzoneidbyname(3C)](https://illumos.org/man/3C/getzoneidbyname), and
[getzonenamebyid(3C)](https://illumos.org/man/3C/getzonenamebyid) for illumos based
systems.

Example
-------

``` rust
extern crate zonename;

fn main() {
    // Max zonename length
    let zonename_max = zonename::ZONENAME_MAX;
    println!("zonename_max = {}", zonename_max);

    // Get the current zone id
    let zoneid = zonename::getzoneid().expect("failed to get zoneid");
    println!("getzoneid() = {}", zoneid);

    // Get the zonename for the current zone id (our zonename)
    let zonename = zonename::getzonenamebyid(zoneid).expect("failed to get zonename");
    println!("getzoneidbyname({}) = {}", zoneid, zonename);

    // Convenience wrapper for getting our current zonename
    let zonename = zonename::getzonename().expect("failed to get zonename");
    println!("getzonename() = {}", zonename);

    // Try to get a fake zoneid
    let zoneid = -1;
    match zonename::getzonenamebyid(zoneid) {
        Ok(name) => println!("getzonenamebyid({}) = name: {}", zoneid, name),
        Err(err) => println!("getzonenamebyid({}) = err: {}", zoneid, err)
    };

    // Try to get a fake zonename
    let zonename = "fake zone";
    match zonename::getzoneidbyname(zonename) {
        Ok(num) => println!("getzoneidbyname('{}') = num: {}", zonename, num),
        Err(err) => println!("getzoneidbyname('{}') = err: {}", zonename, err)
    };
}
```

Yields

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/zonename`
zonename_max = 64
getzoneid() = 47
getzoneidbyname(47) = 5a88b137-93ba-4080-f5ca-d20fecad59e3
getzonename() = 5a88b137-93ba-4080-f5ca-d20fecad59e3
getzonenamebyid(-1) = err: Invalid argument
getzoneidbyname('fake zone') = err: Invalid argument
```

Usage
-----

``` rust
pub const ZONENAME_MAX: usize = 64;
pub fn getzoneid() -> Result<i32>;
pub fn getzoneidbyname(zonename: &str) -> Result<i32>;
pub fn getzonenamebyid(id: i32) -> Result<String>;
pub fn getzonename() -> Result<String>;
```

Todo
----

1. are the types correct? is this code correct?
2. publish to crates?
