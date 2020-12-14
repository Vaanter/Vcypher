# Vcypher [![Build Status](https://travis-ci.com/WTroll/Vcypher.svg?token=wNhJm4Vqfech3FD7SMeb&branch=master)](https://travis-ci.com/WTroll/Vcypher)

*Simple algorithm for deterministic number generation from input.*

This algorithm can be used while learning a new language as it utilizes multiple basic concepts.

This repo contains implementation in Python, Rust, Java and Dart.


## How does vcypher work
<ol>
<li>Get binary representation of each character in input with at least 8 bits.</li>
<li>Count 0s separated by 1s and add previous count to it (ignore if first count).</li>
<li>Count all 1s per character, multiple them together, remove trailing 0s. Append it to the end.</li>
<li>Reverse every number.</li>
<li>Move last number to the start.</li>
<li>*Repeat until all numbers moved* Move first 2 then last 2.</li>
<li>Concatenate everything together.</li>
</ol>

#### Example
Input: ABC
<ol>
<li> 01000001 01000010 01000011 </li>
<li> 1 5 1 4 2 4 -> 1 6 7 11 13 17 </li>
<li> 2 2 3 -> 12 </li>
<li> 1 6 7 11 13 17 12 -> 1 6 7 11 31 71 21 </li>
<li> 21 1 6 71 31 7 11 -> 21167131711 </li>
</ol>

## Getting started
* Python
>  *Python 3.8 or higher is required*
> ```sh
> python3 main.py <your input> (--base64)
> ```

* Java
> *Java 11 or higher is required*\
>
> Building:
> ```sh
> ./gradlew build
> ```
> Running:
> ```sh
> java -jar /path/to/jar/Vcypher-<$version>.jar <your input>
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

## Problems
Collisions are common. 

## Contribution
Any contributions are welcome.
