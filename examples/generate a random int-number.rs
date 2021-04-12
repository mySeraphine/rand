// // In rand version=0.7.3 or lower
// // If you want to generate a random integer number between 1 and 100,
// // like writing in the book "The Rust Programming Language"
// // you can write like this:
// use rand::Rng;
// fn main(){
//     let rand_int_number = rand::thread_rng().gen_range(1, 101);
// }

// But in "rand" crate version=0.8.0 or greater, when compiling the code above,
// it will report an error below:
// |     let secret_number = rand::thread_rng().gen_range(1,101);
// |                                            ^^^^^^^^^ - --- supplied 2 arguments
// |                                            |
// |                                            expected 1 argument


// This is because the parameters of function "gen_range()" changed since version "0.8.0".
// If rand crates version is "0.8.3" or greater,
// You can generate a number between 1 and 100 like following:
use rand::Rng;
fn main()
{
let rand_int_number = rand::thread_rng().gen_range(1..101);
}
