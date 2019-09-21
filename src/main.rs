mod opt;
mod misc;
mod encode;
mod mini_lock;

use opt::Options;
use rust_util::util_msg::*;

fn main() {
    let options = Options::parse_args_static();

    if options.verbose {
        print_message(MessageType::DEBUG, &format!("Run {} in verbose mode", misc::NAME));
        print_message(MessageType::DEBUG, &format!("{:?}", &options));
    }

    if options.version {
        misc::print_version();
        return;
    }

    println!();
    misc::print_help();
    println!();
    println!("{:?}", mini_lock::MINI_LOCK_MAGIC_BYTES);
}
