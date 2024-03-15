# Brainfuck-Interpreter
A simple interpreter for the esoteric language brainfuck written in rust!

## Usage
If using cargo \
`cargo run {path_to_brainfuck_file.txt} {memory array size}`
\
\
If you built the executable
\
`./brainf_interpreter.exe {path_to_file.txt} {memory array size}`\
\
For example: \
\
`./brainf_interpreter.exe ./example.brainf 128` \
**OR**\
\
`cargo run ./example.brainf 128`

## Requirements
Rustc and cargo installed on your machine

## Installation
Clone the repository and open the root directory \
Run `cargo build --release` to build the program into an executable \
 **OR** run `cargo run` to run the program without building it \

If you built the executable, it can be found in the /target/ directory \
