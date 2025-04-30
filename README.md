## Get-Proc-Address-rs
This crate exposes a rust native implementation of the windows `GetProcAddress` function. No external dependencies and no WinApi calls are used.

Tested to work on both `x86` and `x86_64` architectures. 

This crate is filled with unsafe code as we are handling raw pointers.
