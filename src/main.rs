extern crate structopt;
extern crate users;

mod changes;
mod file;
mod libc_error;
mod mode;
mod options;
mod report;
mod target;

use changes::Change;
use file::File;
use options::Options;
use report::Report;
use target::Target;


fn main() {
    let options = Options::new();
    let report = Report::new(&options);

    for target in options.targets() {
        match target {
            Target::Found(ref file)  => change_file(file, &options, &report),
            Target::Missing(name) => report.missing_target(name),
        }
    }
}

fn change_file(file: &File, options: &Options, report: &Report) {
    let changes = file.changes(&options);
    if changes.is_empty() {
        report.no_changes(file);
    } else {
        let applied_changes = changes.iter()
            .map(Change::apply)
            .collect();
        report.changes_applied(file, applied_changes);
    }
}
