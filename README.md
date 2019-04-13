# ch

A command line utility for changing file properties.

`ch` combines the functions of `chown`, `chgrp` and `chmod` into a single
utility that can change the owner, group and mode properties of files simultaneously.

## Usage

    ch [FLAGS] [OPTIONS] [FILES]...

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v, --verbose    Be verbose, show each modified file; specify twice to list old 
                         and new file properties, thrice to show absolute paths

    OPTIONS:
        -g, --group <group>    Change the group of FILES to this group name or numeric ID
        -m, --mode <mode>      Change the mode bits of FILES to this octal or symbolic mode
        -o, --owner <owner>    Change the owner of FILES to this user name or numeric ID

    ARGS:
        <FILES>...    

## License

`ch` is made available under a BSD-style license; see the `LICENSE` file for
details.

