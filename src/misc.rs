
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn print_version() {
    println!("{} - {}", NAME, VERSION);
}

pub fn print_help() {
    print_version();
    println!("{} implements {}", NAME, "https://github.com/kaepora/miniLock");
}

