fn main() {
    println!("I am {}",
             if ::std::env::args()
                    .map(|s| s == "blue" || s == "b")
                    .fold(false, ::std::ops::BitXor::bitxor) {
                 "blue"
             } else {
                 "red"
             });
}
