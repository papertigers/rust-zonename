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
