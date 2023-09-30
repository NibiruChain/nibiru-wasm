// system_info.rs

use std::fmt;

use crate::{
    bash::{self, which_ok},
    errors::SystemInfoError,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SystemInfo {
    os: OS,
    arch: Arch,
    curl: Curl,
}

impl SystemInfo {
    pub fn new() -> Result<SystemInfo, SystemInfoError> {
        let curl = SystemInfo::get_curl_variant()?;
        let (os, arch) = SystemInfo::get_os_and_arch()
            .map_err(|e| SystemInfoError::BashError { bash_err: e })?;

        Ok(SystemInfo { curl, os, arch })
    }

    pub fn release_asset_url(
        self,
        binary: crate::tools::Binary,
        version: String,
    ) -> Result<String, SystemInfoError> {
        crate::tools::release_asset_url(binary, self.os, self.arch, version)
    }

    fn get_curl_variant() -> Result<Curl, SystemInfoError> {
        let insecure: bool =
            std::env::var("INSECURE").unwrap_or_default() == "true";

        if which_ok("curl") {
            if !insecure {
                Ok(Curl::Curl)
            } else {
                Ok(Curl::CurlInsecure)
            }
        } else if which_ok("wget") {
            if !insecure {
                Ok(Curl::Wget)
            } else {
                Ok(Curl::WgetInsecure)
            }
        } else {
            return Err(SystemInfoError::CurlVariantUnknown);
        }
    }

    pub fn get_os_and_arch() -> Result<(OS, Arch), crate::errors::BashError> {
        let os = bash::run_bash("uname -s".to_string())?.stdout;
        let os = match os.trim() {
            "Darwin" => OS::Darwin,
            "Linux" => OS::Linux,
            _ => OS::Unknown(os),
        };

        let arch = bash::run_bash("uname -m".to_string())?.stdout;
        let arch = if arch.trim().contains("arm64") {
            Arch::Arm64
        } else if arch.contains("64") {
            Arch::Amd64
        } else if arch.contains("arm") {
            Arch::Arm
        } else if arch.contains("386") {
            Arch::X86
        } else {
            Arch::Unknown(arch.to_string())
        };

        Ok((os, arch))
    }
}

#[derive(Debug, Clone)]
pub enum Curl {
    Curl,
    CurlInsecure,
    Wget,
    WgetInsecure,
}

impl Curl {
    pub fn cmd(self) -> &'static str {
        match self {
            Curl::Curl => "curl",
            Curl::CurlInsecure => "curl --insecure",
            Curl::Wget => "wget",
            Curl::WgetInsecure => "wget --no-check-certificate",
        }
    }
}

/// OS: Operating system. The primary software that manages the computer
/// hardware. It provides a set of services to system users, a runtime
/// environment for applications, and command-line utilities.
#[derive(Debug, Clone)]
pub enum OS {
    /// For MacOS
    Darwin,
    /// For Ubuntu, WSL, ArchLinux, etc.
    Linux,
    /// For Windows and other unsupported operating systems
    Unknown(String),
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            OS::Darwin => "Darwin",
            OS::Linux => "Linux",
            OS::Unknown(s) => s, // If you want to return the string directly
        };
        write!(f, "{}", repr)
    }
}

impl OS {
    pub fn is_known(self) -> bool {
        match self {
            OS::Darwin => true,
            OS::Linux => true,
            OS::Unknown(_os) => false,
        }
    }

    pub fn system_string(self: OS, cpu_arch: Arch) -> String {
        system_string(self, cpu_arch)
    }
}

fn system_string(os: OS, cpu_arch: Arch) -> String {
    format!("{}_{}", os, cpu_arch)
}

/// Arch: Refers to the computer architecture. Typically, refering to the CPU
/// architecture. The `uname -m` command returns the machine hardware name (CPU
/// type).
#[derive(Debug, Clone)]
pub enum Arch {
    Arm64,
    Amd64,
    Arm,
    X86,
    Unknown(String),
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Arch::Arm64 => "arm64",
            Arch::Amd64 => "amd64",
            Arch::Arm => "arm",
            Arch::X86 => "x86",
            Arch::Unknown(s) => s, // If you want to return the string directly
        };
        write!(f, "{}", repr)
    }
}

impl Arch {
    /// expect_release_artifact: We only expect release binaries for the 4
    /// combinations of operating system (OS) and CPU architecture (Arch) made up by:
    /// OS::Darwin, OS::Linux, Arch::Arm64, Arch::Amd64.
    pub fn expect_release_artifact(self, os: OS) -> bool {
        if !os.is_known() {
            return false;
        }
        match self {
            Arch::Arm64 => true,
            Arch::Amd64 => true,
            Arch::Arm => false,
            Arch::X86 => false,
            Arch::Unknown(_) => false,
        }
    }

    pub fn system_string(self: Arch, os: OS) -> String {
        system_string(os, self)
    }
}
