// tools.rs: Provides a wrapper around external Nibiru dev tooling.

use std::fmt;

use crate::{bash::which_ok, errors::SystemInfoError, gh_release};

/// Executable binary in the nibiru_dev toolset
#[derive(Clone, Debug)]
pub enum Binary {
    Nibid,
    Pricefeeder,
}

impl Binary {
    pub fn repo(&self) -> &'static str {
        match self {
            Binary::Nibid => "NibiruChain/nibiru",
            Binary::Pricefeeder => "NibiruChain/pricefeeder",
        }
    }

    pub fn is_installed(&self) -> bool {
        which_ok(self.to_string().as_str())
    }

    /// fallback_version: Version to use in the event that
    /// `Binary::fetch_latest_release_info` fails or has returns only
    /// prereleases.
    pub fn fallback_version(&self) -> &'static str {
        match self {
            Binary::Nibid => "v0.21.11",
            Binary::Pricefeeder => "v0.21.6",
        }
    }

    pub fn fetch_latest_version(&self) -> Result<String, anyhow::Error> {
        let release = self.fetch_latest_release_info()?;
        Ok(release.name)
    }

    /// fetch_version: Fetches the latest release version using the GitHub API
    /// or returns an identifier for a known fallback version.
    pub fn fetch_version(&self) -> String {
        self.clone()
            .fetch_latest_version()
            .map_or(self.clone().fallback_version().to_string(), |v| v)
    }

    pub fn fetch_latest_release_info(
        &self,
    ) -> Result<gh_release::GitHubRelease, anyhow::Error> {
        let repo_str = self.to_string();
        let mut repo_parts = repo_str.split('/');
        let repo_owner = repo_parts.next().expect("expect repo owner");
        let repo_name = repo_parts.next().expect("expect repo name");

        let all_releases = gh_release::fetch_latest_releases(
            repo_owner.to_string(),
            repo_name.to_string(),
        )?;
        let release = all_releases.iter().find(|rel| !rel.prerelease);

        match release {
            Some(r) => Ok(r.clone()),
            None => Err(SystemInfoError::FailedToFetchLatestRelease {
                binary: self.clone(),
            }
            .into()),
        }
    }

    pub fn is_file_instance_of_tool(&self, fname: &str) -> bool {
        fname.contains(&self.to_string())
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Binary::Nibid => "nibid",
            Binary::Pricefeeder => "pricefeeder",
        };
        write!(f, "{}", repr)
    }
}

pub fn release_asset_url(
    binary: Binary,
    os: crate::system_info::OS,
    cpu_arch: crate::system_info::Arch,
    version: String,
) -> Result<String, SystemInfoError> {
    if !cpu_arch.clone().expect_release_artifact(os.clone()) {
        return Err(SystemInfoError::NoReleaseArtifact { os, cpu_arch });
    }

    let system = os.system_string(cpu_arch);
    let repo = binary.clone().repo();
    let ver: String = version
        .strip_prefix('v')
        .map_or(version.clone(), |v| v.to_string());

    Ok( format!(
        "https://github.com/{repo}/releases/download/v{ver}/{binary}_{ver}_{system}", repo=repo, ver=ver, binary=binary, system=system)
        )
}
