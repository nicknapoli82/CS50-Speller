# CS50-Speller
A rust version of CS50s Speller program

This version still uses the original, unmodified speller.c and dictionary.h files. 

I did this to work through some of the pains of FFI in Rust, and just to make sure I knew how it works.

Compilation is a 2 step task.
First compile dictionary.h to an object file:

rustc -O --emit obj --crate-type staticlib dictionary.rs

You should now have a dictionary.o file

Next compile with your version of c compiler speller.c and dictionary.o, you will need to locate the shared object file for the rust standard in order to give the linker all the bindings. I can't give you a direct path because rust appends the filename with a hash. The command I used is: (On a Linux Machine)

clang -o2 speller.c dictionary.o ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/libstd-89cf9eb8d404bb7b.so -o speller

As you can see it is still a very simple command, but you need to find the libstd-*** file. It should be in your home directory, or on Windows it should be in $HOME/.multirust... I think.

Aside from that. Enjoy.
