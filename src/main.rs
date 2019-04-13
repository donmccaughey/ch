mod file;
mod options;
mod mode;
mod monitor;
mod target;

use options::Options;
use monitor::Monitor;


fn main() {
    let options = Options::new();
    let monitor = Monitor::new(&options);

    for target in options.targets() {
        if let Some(ref file) = target.file {
            match file.change_properties(&options) {
                Ok(_) => monitor.changed_target(&target, file),
                Err(ref error) => monitor.error_changing_target(error, &target, file),
            }
        } else {
            monitor.missing_target(&target);
        }
    }
}
