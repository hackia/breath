use crate::api::github;
use crate::api::github::get_github_issues;
use crate::commit::Config;
use std::fs::read_to_string;
use std::io::Error;

///
/// # Parses breathes.toml and returns a vector of issues
/// # Panics
/// on failed get issues
/// on bad breathes.toml format
/// # Errors
/// on failed get issues
/// on bad breathes.toml format
///
pub async fn get_issues() -> Result<Vec<github::Issue>, Error> {
    let config: Config = toml::from_str(
        read_to_string("breathes.toml")
            .expect("failed to read config")
            .as_str(),
    )
    .expect("bad breathes.toml");
    let issue = get_github_issues(config.me.as_str(), config.repository.as_str())
        .await
        .expect("failed to get issues");
    Ok(issue)
}
