# xorpulse

A simple implementation of a bitwise XOR encryption utility written in Rust.

## Description

This program accepts two strings from the user: a plaintext message and an encryption key. It processes both inputs concurrently, applying a bitwise XOR operation character by character. If the encryption key is shorter than the plaintext, the program automatically cycles the key continuously until the entire message is processed. 

Because XOR operations often produce non-printable control characters, the final output is safely displayed in both decimal and hexadecimal formats.

<img width="569" height="277" alt="image" src="https://github.com/user-attachments/assets/e68d6f62-6bf0-476e-84d3-e84623e294b7" />


## Features

* Consumes input streams via standard input (stdin).
* Automatically loops and stretches short keys using iterator cycling.
* Eliminates trailing newline artifacts from user input fields.
* Formats output safely into raw decimal arrays and double-digit hexadecimal streams.

  <img width="388" height="258" alt="image" src="https://github.com/user-attachments/assets/a268cf74-5157-4402-908b-a873e5dd7249" />


## Requirements

* Rust (Cargo toolchain and rustc)

## How to Run

1. Open your terminal in the project root directory.
2. Build and execute the program using Cargo:
   ```bash
   cargo run
