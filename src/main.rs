#![allow(unused)]
fn main() {
    /*
     * Rust support few different varieties of comments
     * 1: Regular comments - which are ignored by the compiler
     *      1: // Line comments which go to the end of the line.
     *      2: /* Block comments which go to the closing delimiter. */
     *
     * 2: Doc Comments - which are parsed into HTML library documentaions
     *      1: /// Generate library docs for the following item.
     *      2: //! Generate library docs for the enclosing item.
     */

    // This is an example of line comment, there are two slashes at the beginning of the line, and nothing writtern after these will be read by the compiler.

    let lucky_number = 7; // comments also be placed at the end of line containing code

    /* You can manuplate expressions more easily with block comments */

    let x = 5 + /*  90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
