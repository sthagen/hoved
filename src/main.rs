fn main() {
    if let Err(e) = hoved::get_args().and_then(hoved::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
