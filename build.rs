use std::process::Command;


#[cfg(target_os = "linux")]
fn main() {
    Command::new("bash").arg("cpyscript.sh").output();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("Whatever the fuck you're using is not supported. This is a Linux program. Kys");
}
