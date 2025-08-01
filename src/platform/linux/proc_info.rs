use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(super) struct ProcInfo {
    pid: u32,
    name: String,
    path: String,
}

impl ProcInfo {
    fn new(pid: u32, name: String, path: String) -> Self {
        ProcInfo { pid, name, path }
    }

    pub(super) fn pid(&self) -> u32 {
        self.pid
    }

    pub(super) fn name(&self) -> String {
        self.name.clone()
    }

    pub(super) fn path(&self) -> String {
        self.path.clone()
    }

    pub(super) fn from_file(mut file: File) -> crate::Result<Self> {
        // read in entire thing, this is only going to be 1 line
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let line = String::from_utf8_lossy(&buf);
        let buf = line.trim();

        // find the first opening paren, and split off the first part (pid)
        let start_paren = buf.find('(').ok_or("Failed to find opening paren")?;
        let end_paren = buf.rfind(')').ok_or("Failed to find closing paren")?;
        let pid_s = &buf[..start_paren - 1];
        let name = buf[start_paren + 1..end_paren].to_string();

        let exe_path = format!("/proc/{pid_s}/exe");
        let path = fs::read_link(exe_path)
            .unwrap_or(PathBuf::new())
            .to_string_lossy()
            .to_string();

        let pid = FromStr::from_str(pid_s)?;

        Ok(ProcInfo::new(pid, name, path))
    }
}
