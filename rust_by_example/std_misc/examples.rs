fn main() {
    threads::spawn_and_join();
    threads::map_reduce();
    threads::channels();
    path_from_string();
    io::create();
    io::open();
    io::read_lines();
    processes::call_process();
    processes::pipes();
    processes::wait();
    filesystem::ops();
    args::parse_args();
    ffi::call_c();

}
mod threads {
    use std::thread;

    const NTHREADS: i32 = 10; 

    // This is the `main` thread
    pub fn spawn_and_join() {
        println!("\nTHREADS - SPAWN AND JOIN:");
        // Make a vector to hold the children which are spawned.
        let mut children = vec![];

        for i in 0..NTHREADS {
            // Spin up another thread
            children.push(thread::spawn(move || {
                println!("this is thread number {}", i);
            }));
        }

        for child in children {
            // Wait for the thread to finish. Returns a result.
            let _ = child.join();
        }
    }

    // This is the `main` thread
    pub fn map_reduce() {
        println!("\nTHREADS - MAPREDUCE:");

        // This is our data to process.
        // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
        // Each whitespace separated chunk will be handled in a different thread.
        //
        // TODO: see what happens to the output if you insert spaces!
        let data = "86967897737416471853297327050364959
    11861322575564723963297542624962850
    70856234701860851907960690014725639
    38397966707106094172783238747669219
    52380795257888236525459303330302837
    58495327135744041048897885734297812
    69920216438980873548808413720956532
    16278424637452589860345374828574668";

        // Make a vector to hold the child-threads which we will spawn.
        let mut children = vec![];

        /*************************************************************************
         * "Map" phase
         *
         * Divide our data into segments, and apply initial processing
         ************************************************************************/

        // split our data into segments for individual calculation
        // each chunk will be a reference (&str) into the actual data
        let chunked_data = data.split_whitespace();

        // Iterate over the data segments.
        // .enumerate() adds the current loop index to whatever is iterated
        // the resulting tuple "(index, element)" is then immediately
        // "destructured" into two variables, "i" and "data_segment" with a
        // "destructuring assignment"
        for (i, data_segment) in chunked_data.enumerate() {
            println!("data segment {} is \"{}\"", i, data_segment);

            // Process each data segment in a separate thread
            //
            // spawn() returns a handle to the new thread,
            // which we MUST keep to access the returned value
            //
            // 'move || -> u32' is syntax for a closure that:
            // * takes no arguments ('||')
            // * takes ownership of its captured variables ('move') and
            // * returns an unsigned 32-bit integer ('-> u32')
            //
            // Rust is smart enough to infer the '-> u32' from
            // the closure itself so we could have left that out.
            //
            // TODO: try removing the 'move' and see what happens
            children.push(thread::spawn(move || -> u32 {
                // Calculate the intermediate sum of this segment:
                let result = data_segment
                            // iterate over the characters of our segment..
                            .chars()
                            // .. convert text-characters to their number value..
                            .map(|c| c.to_digit(10).expect("should be a digit"))
                            // .. and sum the resulting iterator of numbers
                            .sum();

                // println! locks stdout, so no text-interleaving occurs
                println!("processed segment {}, result={}", i, result);

                // "return" not needed, because Rust is an "expression language", the
                // last evaluated expression in each block is automatically its value.
                result

            }));
        }


        /*************************************************************************
         * "Reduce" phase
         *
         * Collect our intermediate results, and combine them into a final result
         ************************************************************************/

        // combine each thread's intermediate results into a single final sum.
        //
        // we use the "turbofish" ::<> to provide sum() with a type hint.
        //
        // TODO: try without the turbofish, by instead explicitly
        // specifying the type of final_result
        let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

        println!("Final sum result: {}", final_result);
    }

    
    
    pub fn channels() {
        println!("\nTHREADS - CHANNELS:");
        use std::sync::mpsc::{Sender, Receiver};
        use std::sync::mpsc;
        // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
        // where `T` is the type of the message to be transferred
        // (type annotation is superfluous)
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut children = Vec::new();
    
        for id in 0..NTHREADS {
            // The sender endpoint can be copied
            let thread_tx = tx.clone();
    
            // Each thread will send its id via the channel
            let child = thread::spawn(move || {
                // The thread takes ownership over `thread_tx`
                // Each thread queues a message in the channel
                thread_tx.send(id).unwrap();
    
                // Sending is a non-blocking operation, the thread will continue
                // immediately after sending its message
                println!("thread {} finished", id);
            });
    
            children.push(child);
        }
    
        // Here, all the messages are collected
        let mut ids = Vec::with_capacity(NTHREADS as usize);
        for _ in 0..NTHREADS {
            // The `recv` method picks a message from the channel
            // `recv` will block the current thread if there are no messages available
            ids.push(rx.recv());
        }
        
        // Wait for the threads to complete any remaining work
        for child in children {
            child.join().expect("oops! the child thread panicked");
        }
    
        // Show the order in which the messages were sent
        println!("{:?}", ids);
    }

}



fn path_from_string() {
    println!("\nCREATE PATH FROM STRING:");
    use std::path::Path;
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    
    

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    let mut new_path = path.join("a").join("b");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates the file name of the `PathBuf`
    new_path.set_file_name("package.tgz");

    // The `display` method returns a `Display`able structure
    let display = new_path.display();
    println!("{}",display);

    // Convert the `PathBuf` into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

mod io {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::io::{self, BufRead};

    const FILENAME: &str = "lorem_ipsum.txt";

    pub fn create() {
        println!("\nIO - CREATE FILE:");
        static LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
    quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
    consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
    cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
    proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
    ";
        let path = Path::new(FILENAME);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }



    pub fn open() {
        println!("\nIO - OPEN FILE:");
        // Create a path to the desired file
        let path = Path::new(FILENAME);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        // `file` goes out of scope, and the "hello.txt" file gets closed
    }

    

    pub fn read_lines() {
        println!("\nIO - READ LINES:");
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_line_buffer(FILENAME) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_line_buffer<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

mod processes {
    use std::process::{Command, Stdio};

    pub fn call_process() {
        println!("\nPROCESSES - CALL NEW:");
        let output = Command::new("rustc")
            .arg("--version")
            .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
        });

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);

            print!("rustc succeeded and stdout was:\n{}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);

            print!("rustc failed and stderr was:\n{}", s);
        }
    }

    



    pub fn pipes() {
        println!("\nPROCESSES - PIPES:");
        use std::io::prelude::*;
        static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
        // Spawn the `wc` command
        let process = match Command::new("wc")
                                    .stdin(Stdio::piped())
                                    .stdout(Stdio::piped())
                                    .spawn() {
            Err(why) => panic!("couldn't spawn wc: {}", why),
            Ok(process) => process,
        };

        // Write a string to the `stdin` of `wc`.
        //
        // `stdin` has type `Option<ChildStdin>`, but since we know this instance
        // must have one, we can directly `unwrap` it.
        match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {}", why),
            Ok(_) => println!("sent pangram to wc"),
        }

        // Because `stdin` does not live after the above calls, it is `drop`ed,
        // and the pipe is closed.
        //
        // This is very important, otherwise `wc` wouldn't start processing the
        // input we just sent.

        // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read wc stdout: {}", why),
            Ok(_) => print!("wc responded with:\n{}", s),
        }
    }

    pub fn wait() {
        println!("\nPROCESSES - WAIT:");
        let mut child = Command::new("sleep").arg("5").spawn().unwrap();
        let _result = child.wait().unwrap();
    
        println!("reached end of wait");
    }
}

mod filesystem {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::prelude::*;
    use std::os::unix;
    use std::path::Path;

    // A simple implementation of `% cat path`
    fn cat(path: &Path) -> io::Result<String> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // A simple implementation of `% echo s > path`
    fn echo(s: &str, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;

        f.write_all(s.as_bytes())
    }

    // A simple implementation of `% touch path` (ignores existing files)
    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn ops() {
        println!("\nFILESYSTEM - OPS:");
        println!("`mkdir a`");
        // Create a directory, returns `io::Result<()>`
        match fs::create_dir("a") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }

        println!("`echo hello > a/b.txt`");
        // The previous match can be simplified using the `unwrap_or_else` method
        echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`mkdir -p a/c/d`");
        // Recursively create a directory, returns `io::Result<()>`
        fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`touch a/c/e.txt`");
        touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`ln -s ../b.txt a/c/b.txt`");
        // Create a symbolic link, returns `io::Result<()>`
        if cfg!(target_family = "unix") {
            unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }

        println!("`cat a/c/b.txt`");
        match cat(&Path::new("a/c/b.txt")) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(s) => println!("> {}", s),
        }

        println!("`ls a`");
        // Read the contents of a directory, returns `io::Result<Vec<Path>>`
        match fs::read_dir("a") {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => for path in paths {
                println!("> {:?}", path.unwrap().path());
            },
        }

        println!("`rm a/c/e.txt`");
        // Remove a file, returns `io::Result<()>`
        fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        println!("`rmdir a/c/d`");
        // Remove an empty directory, returns `io::Result<()>`
        fs::remove_dir("a/c/d").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

}


mod args {
    use std::env;

    pub fn parse_args() {
        println!("\nARGS - PARSE:");
        let args: Vec<String> = env::args().collect();

        // The first argument is the path that was used to call the program.
        println!("My path is {}.", args[0]);

        // The rest of the arguments are the passed command line parameters.
        // Call the program like this:
        //   $ ./args arg1 arg2
        println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    }
}

mod ffi {
    use std::fmt;

        // this extern block links to the libm library
        #[link(name = "m")]
        extern {
            // this is a foreign function
            // that computes the square root of a single precision complex number
            fn csqrtf(z: Complex) -> Complex;

            fn ccosf(z: Complex) -> Complex;
        }

        // Since calling foreign functions is considered unsafe,
        // it's common to write safe wrappers around them.
        fn cos(z: Complex) -> Complex {
            unsafe { ccosf(z) }
        }

        pub fn call_c() {
            println!("\nFFI - CALL C:");
            // z = -1 + 0i
            let z = Complex { re: -1., im: 0. };

            // calling a foreign function is an unsafe operation
            let z_sqrt = unsafe { csqrtf(z) };

            println!("the square root of {:?} is {:?}", z, z_sqrt);

            // calling safe API wrapped around unsafe operation
            println!("cos({:?}) = {:?}", z, cos(z));
        }

        // Minimal implementation of single precision complex numbers
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct Complex {
            re: f32,
            im: f32,
        }

        impl fmt::Debug for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.im < 0. {
                    write!(f, "{}-{}i", self.re, -self.im)
                } else {
                    write!(f, "{}+{}i", self.re, self.im)
                }
            }
    }
}
