// => what is env?
//      env is standard rust library that helps in
//      interacting with enviornment variables, cli's etc.
<br>
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

// => what is the error for using mpsc channel?
          mpsc channel is unbounded meaning it can create channels without ever stopping
          what will happen with this? => then amount of the sender sending packets will be much higher than
          receiver receiving those packets, so these packets until will be present in the memory
          taking memory and wasting it.
