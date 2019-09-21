mod ch01_01_no_duplicate;
mod ch01_02_is_permut;
mod ch01_03_urlify;
mod ch01_04_compression;

fn main() {
    println!(
        "{}",
        ch01_01_no_duplicate::is_unique_flex("ABCDEFGHIJKLMNOPQRSTUVWXYZZ")
    );
    println!(
        "{}",
        ch01_01_no_duplicate::is_unique_mutate(&mut String::from("ZABCDEFGHIJKLMNOPQRSTUVWXY"))
    );
    println!(
        "{}",
        ch01_02_is_permut::is_permutation("dormitory", "dirtyroom")
    );
    println!(
        "{}",
         ch01_03_urlify::urlify("www.moo.com/good boy")
         );
    println!(
        "{}", 
        ch01_04_compression::compress("meow")
        );
}
