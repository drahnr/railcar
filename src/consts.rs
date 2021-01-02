use lazy_static::lazy_static;

use ::oci::{LinuxDevice, LinuxDeviceType};
use nix::sched::CloneFlags;
use std::collections::HashMap;
use std::os::unix::io::RawFd;

lazy_static! {
    pub static ref DEFAULT_DEVICES: Vec<LinuxDevice> = {
        let v = vec![
            LinuxDevice {
                path: "/dev/null".to_string(),
                typ: LinuxDeviceType::c,
                major: 1,
                minor: 3,
                file_mode: Some(0o066),
                uid: None,
                gid: None,
            },
            LinuxDevice {
                path: "/dev/zero".to_string(),
                typ: LinuxDeviceType::c,
                major: 1,
                minor: 5,
                file_mode: Some(0o066),
                uid: None,
                gid: None,
            },
            LinuxDevice {
                path: "/dev/full".to_string(),
                typ: LinuxDeviceType::c,
                major: 1,
                minor: 7,
                file_mode: Some(0o066),
                uid: None,
                gid: None,
            },
            LinuxDevice {
                path: "/dev/tty".to_string(),
                typ: LinuxDeviceType::c,
                major: 5,
                minor: 0,
                file_mode: Some(0o066),
                uid: None,
                gid: None,
            },
            LinuxDevice {
                path: "/dev/urandom".to_string(),
                typ: LinuxDeviceType::c,
                major: 1,
                minor: 9,
                file_mode: Some(0o066),
                uid: None,
                gid: None,
            },
            LinuxDevice {
                path: "/dev/random".to_string(),
                typ: LinuxDeviceType::c,
                major: 1,
                minor: 8,
                file_mode: Some(0o066),
                uid: None,
                gid: None,
            },
        ];
        v
    };
}

lazy_static! {
    pub static ref NAMESPACES: HashMap<CloneFlags, &'static str> = {
        let mut result = HashMap::new();
        result.insert(CloneFlags::CLONE_NEWIPC, "ipc");
        result.insert(CloneFlags::CLONE_NEWUTS, "uts");
        result.insert(CloneFlags::CLONE_NEWNET, "net");
        result.insert(CloneFlags::CLONE_NEWPID, "pid");
        result.insert(CloneFlags::CLONE_NEWNS, "mnt");
        result.insert(CloneFlags::CLONE_NEWCGROUP, "cgroup");
        result.insert(CloneFlags::CLONE_NEWUSER, "user");
        result
    };
}

pub const CONFIG: &'static str = "config.json";
pub const INIT_PID: &'static str = "init.pid";
pub const PROCESS_PID: &'static str = "process.pid";
pub const TSOCKETFD: RawFd = 9;
