#[no_mangle]
pub extern "C" fn b() {
    let output = std::process::Command::new("uptime")
        .arg("-p")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    println!("{}", output);
}