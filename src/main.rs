mod opt;
mod misc;

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
}
