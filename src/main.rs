mod changes;
mod file;
mod options;
mod mode;
mod monitor;
mod target;

use options::Options;
use monitor::Monitor;
use target::Target;


fn main() {
    let options = Options::new();
    let monitor = Monitor::new(&options);

    for target in options.targets() {
        match target {
            Target::Found(ref file) => {
                match file.change_properties(&options) {
                    Ok(changes) => monitor.file_was_changed(file, &changes),
                    Err(ref error) => monitor.file_change_failed(file, error),
                }
            },
            Target::Missing(name) => {
                monitor.target_is_missing(name);
            },
        }
    }
}
