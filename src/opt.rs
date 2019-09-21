use argparse::{ArgumentParser, StoreTrue, Store};

#[derive(Debug)]
pub struct Options {
    pub version: bool,
    pub verbose: bool,
    pub file: String,
}

impl Options {
    pub fn new() -> Options {
        Options {
            version: false,
            verbose: false,
            file: String::new(),
        }
    }

    pub fn parse_args_static() -> Options {
        let mut opt = Options::new();
        opt.parse_args();
        opt
    }

    pub fn parse_args(&mut self) {
        let mut ap = ArgumentParser::new();
        ap.set_description("miniLock-rs - command line miniLock tool.");
        ap.refer(&mut self.version).add_option(&["-V", "--version"], StoreTrue, "Print version");
        ap.refer(&mut self.verbose).add_option(&["-v", "--verbose"], StoreTrue, "Verbose");
        ap.refer(&mut self.file).add_argument("FILE", Store, "FILE");
        ap.parse_args_or_exit();
    }
}