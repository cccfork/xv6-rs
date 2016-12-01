# xv6-rs

xv6-rs is a Rust re-implementation
of [xv6](http://pdos.csail.mit.edu/6.828/2016/xv6.html), which is in
turn a re-implementation of Dennis Ritchie's and Ken Thompson's Unix
Version 6 (v6).

## ERROR REPORTS

If you spot errors or have suggestions for improvement, please file an
issue in
the [issue tracker](https://github.com/jeehoonkang/xv6-rs/issues).
Pull requests are very welcome.  If you have suggestions for
improvements, please keep in mind that the main purpose of xv6-rs is a
Rusty re-implementation of xv6.  For example, we are in particular
interested in simplifications and clarifications, instead of
suggestions for new systems calls, more portability, etc.

## BUILDING AND RUNNING XV6

To build xv6 on an x86 ELF machine (like Linux or FreeBSD), run "make".
On non-x86 or non-ELF machines (like OS X, even on x86), you will
need to install a cross-compiler gcc suite capable of producing x86 ELF
binaries.  See http://pdos.csail.mit.edu/6.828/2016/tools.html.
Then run "make TOOLPREFIX=i386-jos-elf-".

You also have to install the Nightly Rust and the relevant tools.
Install [rustup](https://www.rustup.rs/) and execute `rustup override
nightly-i686-unknown-linux-gnu` in the xv6-rs directory.

To run xv6, install the QEMU PC simulators.  To run in QEMU, run "make qemu".
