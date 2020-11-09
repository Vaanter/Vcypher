# Vcypher [![Build Status](https://travis-ci.com/WTroll/Vcypher.svg?token=wNhJm4Vqfech3FD7SMeb&branch=master)](https://travis-ci.com/WTroll/Vcypher)

*Simple algorithm for deterministic number generation from input.*

This algorithm can be used while learning a new language as it utilizes multiple basic concepts.

This repo contains implementation in Python, Rust, Java and Dart.


## How does vcypher work
##### Array should be understood as variable length array like data structure in this context.
> <ol>
> <li>Create 2 arrays.</li>
> <li>Convert each char to byte and append it to array 1.</li>
> <li>Count each bit with value of 1 per byte and append it to array 2.</li>
> <li>Concatenate all bytes from array 1, split the string on 1s, get length of each part.</li>
> <li>Multiply all counts from step 2, remove trailing 0s and append the result to step 3.</li>
> <li>Reverse each number. (Any leading 0 is kept.)</li>
> <li>Pop last value and append it to new variable length array like data structure.</li>
> <li>Pop first value and append it to array from step 6 twice.</li>
> <li>Pop last value and append it to array from step 6 twice.</li>
> <li>Repeat steps 7 and 8 until original array is empty.</li>
> <li>Concatenate all values from array from step 6 and return it.</li>
> </ol> 

## Getting started
* Python
>  *Python 3.8 or higher is required*
> ```sh
> python3 main <your input>
> ```

* Java
> *Java 9 or higher is required*\
>
> Building:
> ```sh
> ./gradlew shadowJar
> ```
> Running:
> ```sh
> ./path/to/jar/Vcypher-<$version>.jar <your input>
> ```

* Rust
>  *Rust 1.47.0 has been tested but other versions might work*
> Building:
> ```sh
> cargo build (--release)
> ```
> Running:
> ```sh
> ./path/to/bin/vcypher(.exe) <your input> (--base64)
> ```

## Contribution
Any contributions are welcome.
