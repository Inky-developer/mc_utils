//! Prints out the .minecraft directory
use mc_utils::minecraft_dir;

fn main() {
    println!("{}", minecraft_dir().display());
}
