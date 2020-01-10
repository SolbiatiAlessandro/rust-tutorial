pub fn run() -> i32{
    let pb = indicatif::ProgressBar::new(100);
    let duration = std::time::Duration::from_millis(100);
    for i in 0..100 {
        std::thread::sleep(duration);
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    return 43;
}
