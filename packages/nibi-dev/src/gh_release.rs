use std::str::FromStr;

use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// The GitHub REST API is versioned. The API version name is base on the date
/// when the API is released. When a new REST API version is released, there are
/// potentially breaking changes, and the previous version will be supported for
/// at least 24 months.
///
/// By versioning our API requests, we help guarantee type safety for this
/// dependency API dependency.
///
/// To check if a version is active, use the `X-GitHub-Api-Version` header:
/// ```bash
/// curl --header "X-GitHub-Api-Version:2022-11-28" https://api.github.com/zen
/// ```
///
/// - See ["Meta"](https://docs.github.com/en/rest/meta#get-all-api-versions) for
/// the full list of supported API versions.
pub static GITHUB_API_VERSION: GitHubApiVersion = GitHubApiVersion {
    full_header: "X-GitHub-Api-Version:2022-11-28",
    header_name: "X-GitHub-Api-Version",
    version: "2022-11-28",
};

#[allow(dead_code)]
pub struct GitHubApiVersion {
    version: &'static str,
    header_name: &'static str,
    full_header: &'static str,
}

impl GitHubApiVersion {
    pub fn to_req_header_map(&self) -> anyhow::Result<header::HeaderMap> {
        let mut headers = header::HeaderMap::new();
        let header_key =
            header::HeaderName::from_str(GITHUB_API_VERSION.header_name)?;
        let header_value =
            header::HeaderValue::from_str(GITHUB_API_VERSION.version)?;
        headers.insert(header_key, header_value);
        Ok(headers)
    }
}

/// headers_user_agent: Specifies an HTTP request header for the `User-Agent`.
///
/// All API requests MUST includea a valid `User-Agent` header. Requests with no
/// `User-Agent` header will be rejected. GitHub requests that you use your GitHub
/// username or the name of your application for the `User-Agent` header value.
///
/// https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
pub fn headers_user_agent(
) -> anyhow::Result<(header::HeaderName, header::HeaderValue)> {
    let header_name = header::HeaderName::from_str("User-Agent")?;
    let header_value = header::HeaderValue::from_str(
        "repo:NibiruChain/cw-nibiru_crate::nibi-dev",
    )?;
    Ok((header_name, header_value))
}

/// Use the `X-GitHub-Api-Version` header to check if the crate's abstraction
/// of the GitHub API is up-to-date.
pub fn active_gh_api() -> Result<(), anyhow::Error> {
    let mut headers = GITHUB_API_VERSION.to_req_header_map()?;
    let user_agent = headers_user_agent()?;
    headers.insert(user_agent.0, user_agent.1);

    println!("headers: {:?}", headers);

    let zen_url = "https://api.github.com/status";
    let text = reqwest::blocking::Client::new()
        .get(zen_url)
        .headers(headers)
        .send()?
        .text()?;

    if !text.contains("GitHub lives!") {
        return Err(anyhow::anyhow!(text));
    }
    Ok(())
}

/// GitHubRelease: A GitHub release as defined by the GitHub REST API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubRelease {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: u64,
    pub author: GitHubAuthor,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<ReleaseAsset>,
}

impl FromStr for GitHubRelease {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// GitHubAuthor: GitHub author as defined by the GitHub REST API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubAuthor {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

impl FromStr for GitHubAuthor {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// ReleaseAsset: GitHub release asset as defined by the GitHub REST API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseAsset {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: GitHubAuthor,
    pub content_type: String,
    pub state: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

impl FromStr for ReleaseAsset {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// fetch_latest_releases: Fetches the 30 most recent GitHub releases for a repo
/// For more detailed info, see the [GitHub Docs for "List releases"](https://docs.github.com/en/free-pro-team@latest/rest/releases/releases?apiVersion=2022-11-28#list-releases)
pub fn fetch_latest_releases(
    repo_owner: String,
    repo_name: String,
) -> Result<Vec<GitHubRelease>, anyhow::Error> {
    let gh_url = format!(
        "https://api.github.com/repos/{}/{}/releases",
        repo_owner, repo_name
    );
    println!("gh_url: {:?}", gh_url);

    let mut headers = header::HeaderMap::new();
    let (header_name, header_val) = headers_user_agent()?;
    headers.insert(header_name, header_val);
    let resp_text = reqwest::blocking::Client::new()
        .get(gh_url)
        .headers(headers)
        .send()?
        .text()?;

    let releases: Vec<GitHubRelease> = serde_json::from_str(resp_text.as_str())
        .map_err(|e| {
            let err_msg: serde_json::Value =
                json!({"err": format!("{:?}", e), "resp_text": resp_text});
            let pretty_err: String = serde_json::to_string_pretty(&err_msg)
                .expect("failed to pretty print error message");
            anyhow::anyhow!(pretty_err)
        })?;
    Ok(releases)

    // TODO: handle the raw data case if the serde_json deserialize fails.
    // Ideally, you would still keep the value even if its type does not match
    // exactly and try to access only the bare minimum of fields.
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::gh_release::{
        self, GitHubAuthor, GitHubRelease, ReleaseAsset, GITHUB_API_VERSION,
    };

    #[test]
    fn headers() -> anyhow::Result<()> {
        let _ = GITHUB_API_VERSION.to_req_header_map()?;
        Ok(())
    }

    #[test]
    fn api_version_is_not_stale() -> Result<(), anyhow::Error> {
        match gh_release::active_gh_api() {
            Ok(_) => {}
            Err(err) => {
                // For when you're programming offline.
                assert!(err.to_string().contains("failed to lookup address"))
            }
        };
        Ok(())
    }

    #[test]
    fn fetch_latest_releases() -> anyhow::Result<()> {
        let repo_owner = "NibiruChain".to_string();
        let repo_name = "nibiru".to_string();
        match gh_release::fetch_latest_releases(repo_owner, repo_name) {
            Ok(_) => {}
            Err(err) => {
                // For when you're programming offline.
                assert!(err.to_string().contains("failed to lookup address"))
            }
        };
        Ok(())
    }

    #[test]
    fn parse_github_author() -> anyhow::Result<()> {
        let author_json = r#"
        {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        }"#;

        let _author = GitHubAuthor::from_str(author_json)?;
        Ok(())
    }

    #[test]
    fn parse_github_release_asset() -> anyhow::Result<()> {
        let json = r#"
    {
        "url": "https://api.github.com/repos/NibiruChain/nibiru/releases/assets/128719166",
        "id": 128719166,
        "node_id": "RA_kwDOGxdJvs4HrBk-",
        "name": "nibid_0.21.11_checksums.txt",
        "label": "",
        "uploader": {
          "login": "github-actions[bot]",
          "id": 41898282,
          "node_id": "MDM6Qm90NDE4OTgyODI=",
          "avatar_url": "https://avatars.githubusercontent.com/in/15368?v=4",
          "gravatar_id": "",
          "url": "https://api.github.com/users/github-actions%5Bbot%5D",
          "html_url": "https://github.com/apps/github-actions",
          "followers_url": "https://api.github.com/users/github-actions%5Bbot%5D/followers",
          "following_url": "https://api.github.com/users/github-actions%5Bbot%5D/following{/other_user}",
          "gists_url": "https://api.github.com/users/github-actions%5Bbot%5D/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/github-actions%5Bbot%5D/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/github-actions%5Bbot%5D/subscriptions",
          "organizations_url": "https://api.github.com/users/github-actions%5Bbot%5D/orgs",
          "repos_url": "https://api.github.com/users/github-actions%5Bbot%5D/repos",
          "events_url": "https://api.github.com/users/github-actions%5Bbot%5D/events{/privacy}",
          "received_events_url": "https://api.github.com/users/github-actions%5Bbot%5D/received_events",
          "type": "Bot",
          "site_admin": false
        },
        "content_type": "text/plain; charset=utf-8",
        "state": "uploaded",
        "size": 496,
        "download_count": 166,
        "created_at": "2023-10-02T17:27:29Z",
        "updated_at": "2023-10-02T17:27:30Z",
        "browser_download_url": "https://github.com/NibiruChain/nibiru/releases/download/v0.21.11/nibid_0.21.11_checksums.txt"
    }
        "#;
        let _release_asset = ReleaseAsset::from_str(json)?;
        Ok(())
    }

    #[test]
    fn parse_github_release() -> anyhow::Result<()> {
        let releases_json: String =
            std::fs::read_to_string("fixture/gh_releases.json")?;
        let _json_val: serde_json::Value =
            serde_json::from_str(releases_json.clone().as_str())?;
        let _releases: Vec<GitHubRelease> =
            serde_json::from_str(releases_json.as_str())?;
        Ok(())
    }
}
