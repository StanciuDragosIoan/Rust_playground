/*

#Basics

write program in main.rs file
compile it with:
    rustc main.rs
run it with ./main

run:
    rustup docs --book (to open rust book offline)



#Cargo introduction (Cargo = the package manager for rust)

    run:
        cargo new hello_cargo 

    this command creates a new directory with a Cargo.toml file and a
    src directory in it (the src directory has a main.rs file)
    
    it also initialized a git repo

    run:
        cargo build to compile the code

    this creates a 'target' directory with a bunch of stuff including the 
    compiled code

    next run the code with: ./target/debug/hello_cargo 

    run:
        cargo run (to compile and run the code)

    run:
        cargo check (this checks the code for errors at compile)

    run:
        cargo build --release (to ship code to production)


    a crate is a collection of Rust source code files (a library)
        *there are binary crates (executables) and library crates (libraries)
*/