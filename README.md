# Teensy_Rust
Hardware interface for the Teensy 3.6 written in Rust

I created this repo to explore Micro control and design, and to understand rust. The goal is to build this up to be a usable library for writing programs for the Teensy 3.6 using rust. My aim is for the code to be heavily documented so it can be used for as a reference wwwhen building or exploring rust.

Previous attempts were failures, my discovery is that I was ovewriting the flash config registers, preventing tthe chip from starting.