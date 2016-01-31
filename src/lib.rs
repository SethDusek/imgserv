
pub mod ipfs {
    use std::path::Path;
    use std::process::{Stdio, Command};
    use std::ffi::OsStr;
    use std::io::{Read, Write};
    pub fn ipfs_hash<T: AsRef<[u8]>>(filecont: T) -> String {
        let mut process = Command::new("ipfs")
                              .arg("add")
                              .arg("-n")
                              .arg("-q")
                              .stdin(Stdio::piped())
                              .stdout(Stdio::piped())
                              .spawn()
                              .unwrap();
        let mut hash = String::new();
        process.stdin.unwrap().write_all(filecont.as_ref()).unwrap();
        process.stdout.unwrap().read_to_string(&mut hash);
        hash.trim().to_string()
    }
}
