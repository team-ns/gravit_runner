use std::path::PathBuf;

pub fn launcher_dir(project_name: &str) -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        dirs::data_dir().map(|dir| dir.join(project_name))
    } else if cfg!(target_os = "linux") {
        dirs::home_dir().map(|dir| dir.join(".minecraftlauncher").join(project_name))
    } else if cfg!(target_os = "macos") {
        dirs::home_dir().map(|dir| dir.join("minecraft").join(project_name))
    } else {
        None
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum OsType {
    LinuxX64,
    MacOSX64,
    WindowsX64,
    WindowsX32,
}

impl OsType {
    pub fn is_windows(&self) -> bool {
        match self {
            OsType::WindowsX64 => true,
            OsType::WindowsX32 => true,
            _ => false,
        }
    }

    pub fn is_linux(&self) -> bool {
        match self {
            OsType::LinuxX64 => true,
            _ => false,
        }
    }

    pub fn is_mac_os(&self) -> bool {
        match self {
            OsType::MacOSX64 => true,
            _ => false,
        }
    }

    pub fn get_bitness(&self) -> i32 {
        match self {
            OsType::LinuxX64 => 64,
            OsType::MacOSX64 => 64,
            OsType::WindowsX64 => 64,
            OsType::WindowsX32 => 32,
        }
    }

    pub fn get_os_type(&self) -> &'static str {
        match self {
            OsType::LinuxX64 => "linux",
            OsType::MacOSX64 => "macos",
            OsType::WindowsX64 => "windows",
            OsType::WindowsX32 => "windows",
        }
    }
}

pub fn get_os_type() -> OsType {
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let os_type = OsType::MacOSX64;
    #[cfg(all(target_os = "linux"))]
    let os_type = {
        let info = uname::uname().expect("Can't get os info");

        match info.machine.as_ref() {
            "x86_64" => OsType::LinuxX64,
            _ => panic!("Unsupported linux arch"),
        }
    };
    #[cfg(all(target_os = "windows"))]
    let os_type = {
        use std::mem;
        use winapi::um::sysinfoapi::{GetNativeSystemInfo, SYSTEM_INFO_u_s, SYSTEM_INFO};
        use winapi::um::winnt::{PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_INTEL};

        let mut system_info: SYSTEM_INFO = unsafe { mem::zeroed() };

        unsafe { GetNativeSystemInfo(&mut system_info) };

        let s: &SYSTEM_INFO_u_s = unsafe { system_info.u.s() };

        match s.wProcessorArchitecture {
            PROCESSOR_ARCHITECTURE_INTEL => OsType::WindowsX32,
            PROCESSOR_ARCHITECTURE_AMD64 => OsType::WindowsX64,
            _ => unreachable!(),
        }
    };
    os_type
}
