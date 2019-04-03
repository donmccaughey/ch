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
        if target.exists() {
            target.change_properties();
            monitor.changed_target(&target);
        } else {
            monitor.missing_target(&target);
        }
    }
}
