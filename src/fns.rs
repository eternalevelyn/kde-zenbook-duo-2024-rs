use std::process::Command;
use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
use std::sync::mpsc;
use std::path::Path;

pub fn start() {
    Command::new("kscreen-doctor").arg("output.1.enable").status().unwrap();
    Command::new("kscreen-doctor").arg("output.2.mode.2880x1800@120.00").status().unwrap();
    Command::new("kscreen-doctor").arg("output.2.scale.1.75").status().unwrap();
    Command::new("kscreen-doctor").arg("output.2.disable").status().unwrap();
    Command::new("kscreen-doctor").arg("output.1.mode.2880x1800@120.00").status().unwrap();
    Command::new("kscreen-doctor").arg("output.1.scale.1.75").status().unwrap();



}

pub fn addonautodetect() {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx).unwrap();

    watcher.watch(Path::new("/dev/bus/usb"), RecursiveMode::Recursive).unwrap();

    for res in rx {
        let bool_change = Command::new("bash").arg("checkusb.sh").status().unwrap().success();
        if !bool_change {
            Command::new("kscreen-doctor").arg("output.1.enable").status().unwrap();
            Command::new("kscreen-doctor").arg("output.2.enable").status().unwrap();
        }
        else {
            Command::new("kscreen-doctor").arg("output.eDP-1.enable").status().unwrap();
            Command::new("kscreen-doctor").arg("output.eDP-2.disable").status().unwrap();
        };
    };
}
