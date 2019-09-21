mod ch01_01_no_duplicate;
mod ch01_02_is_permut;

fn main() {
    println!("{}",ch01_01_no_duplicate::is_unique_flex("ABCDEFGHIJKLMNOPQRSTUVWXYZZ"));
    println!("{}",ch01_02_is_permut::is_permutation("dormitory","dirtyroom"));
}
