fn main() {
    println!("I am {}",
             if ::std::env::args()
                    .map(|s| s == "blue")
                    .fold(true, ::std::ops::BitXor::bitxor) {
                 "blue"
             } else {
                 "red"
             });
}
