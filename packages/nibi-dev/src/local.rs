// local.rs: Handles state as specified by the local file system.

use std::{env, fs, path::PathBuf};

use anyhow::anyhow;

use crate::{errors::LocalError, system_info::SystemInfo};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocalState<State = NullState> {
    system_info: SystemInfo,
    root: PathBuf,
    state: std::marker::PhantomData<State>,
}

/// ExistsState: State representing when the temp state is set.
#[derive(Debug, Clone)]
pub struct ExistsState;

/// NullState: State representing when the temp state is not set.
#[derive(Debug, Clone)]
pub struct NullState;

impl LocalState {
    pub fn new() -> Result<LocalState, anyhow::Error> {
        let system_info = SystemInfo::new()?;

        let root = LocalState::ensure_root_exists()?;
        let state = LocalState {
            system_info,
            root: root.clone(),
            state: std::marker::PhantomData,
        };
        LocalState::ensure_state_dirs(root)?;
        state.prepend_bin_to_system_path()?;

        Ok(state)
    }

    pub fn root_path() -> Result<PathBuf, LocalError> {
        let home_dir = match home::home_dir() {
            Some(home_dir) => home_dir,
            None => return Err(LocalError::FailedToFindHomeDir),
        };
        let target_dir = home_dir.join(".local").join("nibiru_dev");
        Ok(target_dir)
    }

    pub fn ensure_root_exists() -> Result<PathBuf, LocalError> {
        let root = LocalState::root_path()?;
        if !root.exists() {
            // Create root
            fs::create_dir_all(&root)
                .map_err(|e| LocalError::InnerError { err: e.into() })?;
        } else if !root.is_dir() {
            return Err(LocalError::FailedToCreateRootDir {
                err: "root path exists but is not a directory",
            });
        }
        Ok(root)
    }

    pub fn ensure_state_dirs(root: PathBuf) -> Result<(), anyhow::Error> {
        let state_dir_names = vec!["bin"];
        for dir_name in &state_dir_names {
            let dir = root.join(dir_name);
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
            // TODO: Handle dir exists but is not directory case.
        }
        Ok(())
    }

    pub fn bin(&self) -> PathBuf {
        self.root.join("bin")
    }

    pub fn prepend_bin_to_system_path(&self) -> anyhow::Result<()> {
        let bin = self.bin();
        let bin_absolute_path: &str = bin.to_str().ok_or_else(|| {
            anyhow!("absolute path to bin is invalid unicode ({:?})", self.bin())
        })?;
        prepend_to_system_path(bin_absolute_path)
    }
}

pub fn prepend_to_system_path(new_path: &str) -> anyhow::Result<()> {
    let new_path_buf = std::path::PathBuf::from(new_path);
    // Retrieve the current PATH variable
    // Convert the PATH variable to a vector of Paths
    let path_var =
        env::var_os("PATH").ok_or_else(|| anyhow!("Error retrieving PATH"))?;

    let mut paths = env::split_paths(&path_var).collect::<Vec<_>>();

    // Remove new_path if it already exists in PATH
    paths.retain(|path| *path != new_path_buf);

    // Add new_path to the beginning of PATH
    paths.insert(0, new_path_buf);

    // Convert the vector of Paths back to a PATH string
    let new_path_var = env::join_paths(paths)
        .map_err(|e| anyhow!("Error joining paths: {:?}", e))?;

    // Set the new PATH variable for the current process
    env::set_var("PATH", new_path_var);

    Ok(())
}

pub mod stateful_tool {

    // State:
    // ? System info OS and Arch known
    // ? Binary versions set
    // ? Binaries installed
    //   ? Unpacked binaries

    use super::LocalState;

    #[derive(Debug, Clone)]
    pub struct ToolInPath;

    #[derive(Debug, Clone)]
    pub struct ToolNotInPath;

    #[derive(Debug, Clone)]
    pub struct StatefulTool<State = ToolNotInPath> {
        tool: crate::tools::Binary,
        state: std::marker::PhantomData<State>,
    }

    impl StatefulTool {
        pub fn new(tool: crate::tools::Binary) -> StatefulTool<ToolNotInPath> {
            StatefulTool {
                tool,
                state: std::marker::PhantomData,
            }
        }

        pub fn is_installed(&self) -> bool {
            self.tool.is_installed()
        }

        pub fn versions_installed(
            &self,
            local: &LocalState,
        ) -> Result<Vec<String>, anyhow::Error> {
            let matching_versions = std::fs::read_dir(local.bin())?
                .filter_map(|dir_entry| {
                    let entry = dir_entry.ok()?;
                    let fname = entry.file_name();
                    let fname_str = fname.to_str()?;
                    if self.clone().tool.is_file_instance_of_tool(fname_str) {
                        return Some(fname_str.to_string());
                    }
                    None
                })
                .collect();
            Ok(matching_versions)
        }

        pub fn clear_bins(&self, local: &LocalState) -> anyhow::Result<()> {
            // Getting the prefix of the binary as a String
            let prefix = self.tool.to_string();

            // Iterate over files in the directory specified by local.bin()
            for entry in std::fs::read_dir(local.bin())? {
                let entry = entry?;
                let path = entry.path();

                // We only want to deal with files, skipping directories
                if path.is_file() {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    // Check if the file_name starts with the prefix
                    if file_name_str.starts_with(&prefix) {
                        // Try to remove the file and propagate errors if any
                        std::fs::remove_file(path)?;
                    }
                }
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tools::Binary;

    use super::*;

    #[test]
    fn test_local_state_new() -> anyhow::Result<()> {
        let state = LocalState::new()?;
        let env_path = env::var("PATH")?;
        let bin_path_buf = state.bin();
        let bin: &str = bin_path_buf
            .to_str()
            .ok_or_else(|| anyhow!("failed to parse bin path"))?;
        assert!(env_path.starts_with(bin));
        Ok(())
    }

    #[test]
    fn test_versions_installed() -> anyhow::Result<()> {
        let local = LocalState::new()?.clone();
        let tools: Vec<Binary> = vec![Binary::Nibid, Binary::Pricefeeder];
        for tool in &tools {
            let s = stateful_tool::StatefulTool::new(tool.clone()).clone();
            let mut vers = s.versions_installed(&local.clone())?;
            if !vers.is_empty() {
                s.clear_bins(&local)?;
            }

            let mock_bin_names: [String; 2] =
                [format!("{}_aaa", tool), format!("{}_bbb", tool)];

            for mock_bin in &mock_bin_names {
                let fname = local.bin().join(mock_bin);
                let _file = std::fs::File::create(fname)?;
            }

            vers = s.versions_installed(&local)?;
            assert!(!vers.is_empty(), "vers: {:?}", vers);
            assert!(vers.iter().all(|ver| mock_bin_names.contains(ver)));
        }
        Ok(())
    }
}
