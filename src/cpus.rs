use std::ffi::OsString;

struct CPU {
    path: OsString,
    max_frecuency: usize,
    min_frecuency: usize,
}

impl CPU {
    fn try_new(path: &OsString) {
        todo!()
    }
}

struct CPUs {
    cpus: Vec<CPU>,
}

impl CPUs {
    fn try_new() {}
}
