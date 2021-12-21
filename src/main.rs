mod tests;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .format_timestamp(None)
        .init();
    println!("Elements of Programming, in Rust!");
    tests::run_tests();
}
