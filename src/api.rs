use std::collections::HashMap;
use std::time::Duration;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, Method, Url};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provider {
    GitHub,
    GitLab,
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub provider: Provider,
    pub token: String,
    pub owner: String,
    pub repo: String,
    /// Optional API base URL. Defaults:
    /// - GitHub: https://api.github.com
    /// - GitLab: https://gitlab.com/api/v4
    pub base_url: Option<String>,
    http: Client,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64, // GitHub number or GitLab iid
    pub title: String,
    pub state: String,
    pub url: String, // html_url (GH) or web_url (GL)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub id: u64,
    pub body: String,
    pub url: Option<String>,
}

impl ApiClient {
    pub fn new(
        provider: Provider,
        token: impl Into<String>,
        owner: impl Into<String>,
        repo: impl Into<String>,
        base_url: Option<String>,
    ) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build http client");
        Self {
            provider,
            token: token.into(),
            owner: owner.into(),
            repo: repo.into(),
            base_url,
            http,
        }
    }

    fn default_base(&self) -> &str {
        match self.provider {
            Provider::GitHub => "https://api.github.com",
            Provider::GitLab => "https://gitlab.com/api/v4",
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        match self.provider {
            Provider::GitHub => {
                // GitHub recommends a UA header
                h.insert(USER_AGENT, HeaderValue::from_static("breath/0.1"));
                let val = format!("Bearer {}", self.token);
                h.insert(AUTHORIZATION, HeaderValue::from_str(&val).unwrap());
                h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            }
            Provider::GitLab => {
                // GitLab uses PRIVATE-TOKEN header
                h.insert(USER_AGENT, HeaderValue::from_static("breath/0.1"));
                h.insert("PRIVATE-TOKEN", HeaderValue::from_str(&self.token).unwrap());
                h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            }
        }
        h
    }

    fn base_url(&self) -> Url {
        let base = self.base_url.as_deref().unwrap_or(self.default_base());
        Url::parse(base).expect("invalid base url")
    }

    fn project_path_gitlab(&self) -> String {
        let id = format!("{}/{}", self.owner, self.repo);
        urlencoding::encode(&id).into_owned()
    }

    async fn request_json<T: serde::de::DeserializeOwned, B: Into<reqwest::Body>>(
        &self,
        method: Method,
        url: Url,
        body: Option<B>,
    ) -> Result<T, String> {
        let mut req = self.http.request(method, url).headers(self.headers());
        if let Some(b) = body {
            req = req.body(b);
        }
        let resp = req.send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("HTTP {status}: {text}"));
        }
        resp.json::<T>().await.map_err(|e| e.to_string())
    }

    // List issues (open by default)
    pub async fn list_issues(&self, state: Option<&str>) -> Result<Vec<Issue>, String> {
        match self.provider {
            Provider::GitHub => {
                let mut url = self.base_url();
                url.set_path(&format!("/repos/{}/{}/issues", self.owner, self.repo));
                {
                    let mut qp = url.query_pairs_mut();
                    if let Some(s) = state {
                        qp.append_pair("state", s);
                    }
                    qp.append_pair("per_page", "100");
                }
                // GitHub returns PRs too; filter them out
                #[derive(serde::Deserialize)]
                struct GhIssue {
                    id: u64,
                    number: u64,
                    title: String,
                    state: String,
                    html_url: String,
                    #[serde(default)]
                    pull_request: Option<HashMap<String, serde_json::Value>>,
                }
                let items: Vec<GhIssue> = self
                    .request_json(Method::GET, url, Option::<reqwest::Body>::None)
                    .await?;
                let issues = items
                    .into_iter()
                    .filter(|i| i.pull_request.is_none())
                    .map(|i| Issue {
                        id: i.id,
                        number: i.number,
                        title: i.title,
                        state: i.state,
                        url: i.html_url,
                    })
                    .collect();
                Ok(issues)
            }
            Provider::GitLab => {
                let mut url = self.base_url();
                url.set_path(&format!("/projects/{}/issues", self.project_path_gitlab()));
                if let Some(s) = state {
                    url.query_pairs_mut().append_pair("state", s);
                }
                url.query_pairs_mut().append_pair("per_page", "100");
                #[derive(serde::Deserialize)]
                struct GlIssue {
                    id: u64,
                    iid: u64,
                    title: String,
                    state: String,
                    web_url: String,
                }
                let items: Vec<GlIssue> = self
                    .request_json(Method::GET, url, Option::<reqwest::Body>::None)
                    .await?;
                let issues = items
                    .into_iter()
                    .map(|i| Issue {
                        id: i.id,
                        number: i.iid,
                        title: i.title,
                        state: i.state,
                        url: i.web_url,
                    })
                    .collect();
                Ok(issues)
            }
        }
    }

    pub async fn get_issue(&self, number: u64) -> Result<Issue, String> {
        match self.provider {
            Provider::GitHub => {
                let mut url = self.base_url();
                url.set_path(&format!(
                    "/repos/{}/{}/issues/{}",
                    self.owner, self.repo, number
                ));
                #[derive(serde::Deserialize)]
                struct GhIssue {
                    id: u64,
                    number: u64,
                    title: String,
                    state: String,
                    html_url: String,
                }
                let i: GhIssue = self
                    .request_json(Method::GET, url, Option::<reqwest::Body>::None)
                    .await?;
                Ok(Issue {
                    id: i.id,
                    number: i.number,
                    title: i.title,
                    state: i.state,
                    url: i.html_url,
                })
            }
            Provider::GitLab => {
                let mut url = self.base_url();
                url.set_path(&format!(
                    "/projects/{}/issues/{}",
                    self.project_path_gitlab(),
                    number
                ));
                #[derive(serde::Deserialize)]
                struct GlIssue {
                    id: u64,
                    iid: u64,
                    title: String,
                    state: String,
                    web_url: String,
                }
                let i: GlIssue = self
                    .request_json(Method::GET, url, Option::<reqwest::Body>::None)
                    .await?;
                Ok(Issue {
                    id: i.id,
                    number: i.iid,
                    title: i.title,
                    state: i.state,
                    url: i.web_url,
                })
            }
        }
    }

    pub async fn create_issue(
        &self,
        title: &str,
        description: Option<&str>,
        labels: &[&str],
    ) -> Result<Issue, String> {
        match self.provider {
            Provider::GitHub => {
                let mut url = self.base_url();
                url.set_path(&format!("/repos/{}/{}/issues", self.owner, self.repo));
                #[derive(serde::Serialize)]
                struct Body<'a> {
                    title: &'a str,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    body: Option<&'a str>,
                    #[serde(skip_serializing_if = "Vec::is_empty")]
                    labels: Vec<&'a str>,
                }
                let b = Body {
                    title,
                    body: description,
                    labels: labels.to_vec(),
                };
                #[derive(serde::Deserialize)]
                struct GhIssue {
                    id: u64,
                    number: u64,
                    title: String,
                    state: String,
                    html_url: String,
                }
                let i: GhIssue = self
                    .request_json(
                        Method::POST,
                        url,
                        Some(reqwest::Body::from(serde_json::to_vec(&b).unwrap())),
                    )
                    .await?;
                Ok(Issue {
                    id: i.id,
                    number: i.number,
                    title: i.title,
                    state: i.state,
                    url: i.html_url,
                })
            }
            Provider::GitLab => {
                let mut url = self.base_url();
                url.set_path(&format!("/projects/{}/issues", self.project_path_gitlab()));
                #[derive(serde::Serialize)]
                struct Body<'a> {
                    title: &'a str,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    description: Option<&'a str>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    labels: Option<String>,
                }
                let labels_joined = if labels.is_empty() {
                    None
                } else {
                    Some(labels.join(","))
                };
                let b = Body {
                    title,
                    description,
                    labels: labels_joined,
                };
                #[derive(serde::Deserialize)]
                struct GlIssue {
                    id: u64,
                    iid: u64,
                    title: String,
                    state: String,
                    web_url: String,
                }
                let i: GlIssue = self
                    .request_json(
                        Method::POST,
                        url,
                        Some(reqwest::Body::from(serde_json::to_vec(&b).unwrap())),
                    )
                    .await?;
                Ok(Issue {
                    id: i.id,
                    number: i.iid,
                    title: i.title,
                    state: i.state,
                    url: i.web_url,
                })
            }
        }
    }

    pub async fn update_issue(
        &self,
        number: u64,
        title: Option<&str>,
        description: Option<&str>,
        labels: Option<&[&str]>,
    ) -> Result<Issue, String> {
        match self.provider {
            Provider::GitHub => {
                // Important: do not permit closing via API; we do not send state=closed
                let mut url = self.base_url();
                url.set_path(&format!(
                    "/repos/{}/{}/issues/{}",
                    self.owner, self.repo, number
                ));
                #[derive(serde::Serialize)]
                struct Body<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    title: Option<&'a str>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    body: Option<&'a str>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    labels: Option<Vec<&'a str>>,
                }
                let b = Body {
                    title,
                    body: description,
                    labels: labels.map(|l| l.to_vec()),
                };
                #[derive(serde::Deserialize)]
                struct GhIssue {
                    id: u64,
                    number: u64,
                    title: String,
                    state: String,
                    html_url: String,
                }
                let i: GhIssue = self
                    .request_json(
                        Method::PATCH,
                        url,
                        Some(reqwest::Body::from(serde_json::to_vec(&b).unwrap())),
                    )
                    .await?;
                Ok(Issue {
                    id: i.id,
                    number: i.number,
                    title: i.title,
                    state: i.state,
                    url: i.html_url,
                })
            }
            Provider::GitLab => {
                let mut url = self.base_url();
                url.set_path(&format!(
                    "/projects/{}/issues/{}",
                    self.project_path_gitlab(),
                    number
                ));
                #[derive(serde::Serialize)]
                struct Body<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    title: Option<&'a str>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    description: Option<&'a str>,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    labels: Option<String>,
                }
                let labels_joined = labels.map(|l| l.join(","));
                let b = Body {
                    title,
                    description,
                    labels: labels_joined,
                };
                #[derive(serde::Deserialize)]
                struct GlIssue {
                    id: u64,
                    iid: u64,
                    title: String,
                    state: String,
                    web_url: String,
                }
                let i: GlIssue = self
                    .request_json(
                        Method::PUT,
                        url,
                        Some(reqwest::Body::from(serde_json::to_vec(&b).unwrap())),
                    )
                    .await?;
                Ok(Issue {
                    id: i.id,
                    number: i.iid,
                    title: i.title,
                    state: i.state,
                    url: i.web_url,
                })
            }
        }
    }

    pub async fn comment_on_issue(&self, number: u64, body: &str) -> Result<Comment, String> {
        match self.provider {
            Provider::GitHub => {
                let mut url = self.base_url();
                url.set_path(&format!(
                    "/repos/{}/{}/issues/{}/comments",
                    self.owner, self.repo, number
                ));
                #[derive(serde::Serialize)]
                struct Body<'a> {
                    body: &'a str,
                }
                #[derive(serde::Deserialize)]
                struct GhComment {
                    id: u64,
                    body: String,
                    html_url: Option<String>,
                }
                let c: GhComment = self
                    .request_json(
                        Method::POST,
                        url,
                        Some(reqwest::Body::from(
                            serde_json::to_vec(&Body { body }).unwrap(),
                        )),
                    )
                    .await?;
                Ok(Comment {
                    id: c.id,
                    body: c.body,
                    url: c.html_url,
                })
            }
            Provider::GitLab => {
                let mut url = self.base_url();
                url.set_path(&format!(
                    "/projects/{}/issues/{}/notes",
                    self.project_path_gitlab(),
                    number
                ));
                #[derive(serde::Serialize)]
                struct Body<'a> {
                    body: &'a str,
                }
                #[derive(serde::Deserialize)]
                struct GlComment {
                    id: u64,
                    body: String,
                }
                let c: GlComment = self
                    .request_json(
                        Method::POST,
                        url,
                        Some(reqwest::Body::from(
                            serde_json::to_vec(&Body { body }).unwrap(),
                        )),
                    )
                    .await?;
                Ok(Comment {
                    id: c.id,
                    body: c.body,
                    url: None,
                })
            }
        }
    }
}

// The project policy: Do NOT close issues via API.
// Instead, we provide a helper that closes via git commit message and push.
// - For GitHub, messages like "Fixes #123" will close the issue when merged into default branch.
// - For GitLab, messages like "Closes #123" have the same effect.
// See: https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue
//      https://docs.gitlab.com/ee/user/project/issues/managing_issues.html#use-commit-messages-to-close-issues
pub fn close_issue_via_git(
    issue_number: u64,
    message: Option<&str>,
    branch: Option<&str>,
) -> Result<(), String> {
    use std::process::Command;
    let close_keywords = ["Fixes", "Closes", "Résout", "Ferme"]; // multilingual, GH/GL recognize Fixes/Closes
    let keyword = close_keywords[0];
    let msg = if let Some(m) = message {
        format!("{} #{}: {}", keyword, issue_number, m)
    } else {
        format!("{} #{}", keyword, issue_number)
    };

    // Ensure working tree is clean enough to allow an empty commit
    let status = Command::new("git")
        .arg("commit")
        .arg("--allow-empty")
        .arg("-m")
        .arg(&msg)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("git commit failed with status {:?}", status.code()));
    }

    let mut push = Command::new("git");
    push.arg("push");
    if let Some(b) = branch {
        push.arg("origin").arg(b);
    }
    let status = push.status().map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("git push failed with status {:?}", status.code()));
    }
    Ok(())
}
