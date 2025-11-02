use reqwest::{Error, header};
use serde::{Deserialize, Serialize};
#[must_use]
pub fn github_token() -> String {
    std::env::var("GITHUB_TOKEN").unwrap_or_else(|_| String::new())
}

// --- Colle les structures Issue et User d'en haut ici ---
#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub state: String,
    pub html_url: String,
    pub body: Option<String>,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GithubIssueType {
    Issue,
    PullRequest,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubIssueRequest {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubIssue {
    pub title: String,
    pub number: u32,
}

///
/// # Errors
/// on request error
/// # Panics
/// on bad JSON
pub async fn get_github_issues(user: &str, repository: &str) -> Result<Vec<Issue>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.github.com/repos/{user}/{repository}/issues").as_str())
        // L'API GitHub REQUIERT un User-Agent. Remplace-le par le nom de ton app.
        .header(header::USER_AGENT, "breath/0.1.0")
        .header(header::ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?;

    let issues = response
        .error_for_status()? // Stoppe ici si le statut n'est pas 2xx
        .json::<Vec<Issue>>() // 4. Désérialiser le JSON en Vec<Issue>
        .await?;

    // 5. Afficher les résultats
    if issues.is_empty() {
        return Ok(vec![]);
    }
    Ok(issues)
}
