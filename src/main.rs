use detect_desktop_environment::DesktopEnvironment;

mod fns;

fn main() {
    let de = match DesktopEnvironment::detect() {
    Some(de) => de,
    None => panic!("Desktop environment detection failed. Don't work right.")
    };
    if de == DesktopEnvironment::Kde {
        fns::start();
        fns::addonautodetect();
    }
    else {
        panic!("Desktop environment not supported. This tool only supports KDE on Linux.");
    };
}
