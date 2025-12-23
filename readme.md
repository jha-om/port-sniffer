// => what is env?
//      env is standard rust library that helps in
//      interacting with enviornment variables, cli's etc.
// => what is args()?
//      args is a module that implements a struct that is
//      providing us an iterator(that gives a single key/value)
//      at a time.
        // trait Iterator {
        //     type Item;  // The type of items being iterated over
        //     fn next(&mut self) -> Option<Self::Item>;  // Get the next item
        // }
// for env::args() => item is string the next() => Option<string>

<!-- when we run -->
<!--cargo run -- -h arg1 arg2-->

<!---- => means that we're telling the rust after -- the arguments are for the executables not for the cargo.-->

-----------------------------------------------------------------------
