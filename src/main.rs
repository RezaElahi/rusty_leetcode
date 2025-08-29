use leetcode::grep_main;

fn main() {
    // test some path
    if let Err(e) = grep_main() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
