extern crate structopt;
extern crate users;

mod changes;
mod file;
mod mode;
mod options;
mod report;
mod target;

use options::Options;
use report::Report;
use target::Target;


fn main() {
    let options = Options::new();
    let report = Report::new(&options);

    for target in options.targets() {
        match target {
            Target::Found(ref file) => {
                match file.change_properties(&options) {
                    Ok(changes) => report.file_was_changed(file, &changes),
                    Err(ref error) => report.file_change_failed(file, error),
                }
            },
            Target::Missing(name) => {
                report.target_is_missing(name);
            },
        }
    }
}
