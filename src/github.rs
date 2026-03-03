use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const GITHUB_GRAPHQL: &str = "https://api.github.com/graphql";

#[derive(Debug, Clone)]
pub struct OrgData {
    pub total_stars: u64,
    pub total_forks: u64,
    pub repo_count: u64,
    pub member_count: u64,
    pub total_commits: u64,
    pub recent_commits: u64,
    pub languages: Vec<LanguageEntry>,
}

#[derive(Debug, Clone)]
pub struct LanguageEntry {
    pub name: String,
    pub color: String,
    pub bytes: u64,
}

#[derive(Deserialize)]
struct GraphQLResponse {
    data: Option<GraphQLData>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Deserialize)]
struct GraphQLData {
    organization: Option<OrgNode>,
}

#[derive(Deserialize)]
struct OrgNode {
    repositories: RepoConnection,
    #[serde(rename = "membersWithRole")]
    members_with_role: CountNode,
}

#[derive(Deserialize)]
struct CountNode {
    #[serde(rename = "totalCount")]
    total_count: u64,
}

#[derive(Deserialize)]
struct RepoConnection {
    #[serde(rename = "totalCount")]
    total_count: u64,
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
    nodes: Vec<RepoNode>,
}

#[derive(Deserialize)]
struct PageInfo {
    #[serde(rename = "hasNextPage")]
    has_next_page: bool,
    #[serde(rename = "endCursor")]
    end_cursor: Option<String>,
}

#[derive(Deserialize)]
struct RepoNode {
    #[serde(rename = "stargazerCount")]
    stargazer_count: u64,
    #[serde(rename = "forkCount")]
    fork_count: u64,
    languages: Option<LanguageConnection>,
    #[serde(rename = "defaultBranchRef")]
    default_branch_ref: Option<BranchRef>,
}

#[derive(Deserialize)]
struct LanguageConnection {
    edges: Vec<LanguageEdge>,
}

#[derive(Deserialize)]
struct LanguageEdge {
    size: u64,
    node: LanguageNode,
}

#[derive(Deserialize)]
struct LanguageNode {
    name: String,
    color: Option<String>,
}

#[derive(Deserialize)]
struct BranchRef {
    target: Option<CommitTarget>,
}

#[derive(Deserialize)]
struct CommitTarget {
    history: Option<HistoryConnection>,
    #[serde(rename = "recentHistory")]
    recent_history: Option<HistoryConnection>,
}

#[derive(Deserialize)]
struct HistoryConnection {
    #[serde(rename = "totalCount")]
    total_count: u64,
}

pub async fn fetch_org_data(client: &Client, token: &str, org: &str) -> Result<OrgData, String> {
    let since = chrono_since_30d();
    let mut total_stars = 0u64;
    let mut total_forks = 0u64;
    let mut repo_count = 0u64;
    let mut member_count = 0u64;
    let mut total_commits = 0u64;
    let mut recent_commits = 0u64;
    let mut lang_map: HashMap<String, (String, u64)> = HashMap::new();

    let mut cursor: Option<String> = None;
    let mut first_page = true;

    loop {
        let query = build_query(&since);
        let variables = if let Some(ref c) = cursor {
            format!(
                r#"{{"login":"{}","first":100,"after":"{}"}}"#,
                org, c
            )
        } else {
            format!(r#"{{"login":"{}","first":100}}"#, org)
        };

        let body = serde_json::json!({
            "query": query,
            "variables": serde_json::from_str::<serde_json::Value>(&variables).map_err(|e| e.to_string())?
        });

        let resp = client
            .post(GITHUB_GRAPHQL)
            .bearer_auth(token)
            .header("User-Agent", "org-stats/0.1")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("GitHub API error {status}: {text}"));
        }

        let gql: GraphQLResponse = resp.json().await.map_err(|e| format!("json parse: {e}"))?;

        if let Some(errors) = gql.errors {
            let msgs: Vec<_> = errors.iter().map(|e| e.message.as_str()).collect();
            return Err(format!("GraphQL errors: {}", msgs.join(", ")));
        }

        let data = gql.data.ok_or("no data in response")?;
        let org_node = data.organization.ok_or("organization not found")?;

        if first_page {
            repo_count = org_node.repositories.total_count;
            member_count = org_node.members_with_role.total_count;
            first_page = false;
        }

        for repo in &org_node.repositories.nodes {
            total_stars += repo.stargazer_count;
            total_forks += repo.fork_count;

            if let Some(ref langs) = repo.languages {
                for edge in &langs.edges {
                    let entry = lang_map
                        .entry(edge.node.name.clone())
                        .or_insert_with(|| {
                            (
                                edge.node.color.clone().unwrap_or_else(|| "#8b8b8b".into()),
                                0,
                            )
                        });
                    entry.1 += edge.size;
                }
            }

            if let Some(ref branch) = repo.default_branch_ref {
                if let Some(ref target) = branch.target {
                    if let Some(ref history) = target.history {
                        total_commits += history.total_count;
                    }
                    if let Some(ref recent) = target.recent_history {
                        recent_commits += recent.total_count;
                    }
                }
            }
        }

        if org_node.repositories.page_info.has_next_page {
            cursor = org_node.repositories.page_info.end_cursor;
        } else {
            break;
        }
    }

    let mut languages: Vec<LanguageEntry> = lang_map
        .into_iter()
        .map(|(name, (color, bytes))| LanguageEntry { name, color, bytes })
        .collect();
    languages.sort_by(|a, b| b.bytes.cmp(&a.bytes));

    Ok(OrgData {
        total_stars,
        total_forks,
        repo_count,
        member_count,
        total_commits,
        recent_commits,
        languages,
    })
}

fn build_query(since: &str) -> String {
    format!(
        r#"query OrgStats($login: String!, $first: Int!, $after: String) {{
  organization(login: $login) {{
    repositories(first: $first, after: $after, isFork: false) {{
      totalCount
      pageInfo {{ hasNextPage endCursor }}
      nodes {{
        stargazerCount
        forkCount
        languages(first: 10, orderBy: {{field: SIZE, direction: DESC}}) {{
          edges {{ size node {{ name color }} }}
        }}
        defaultBranchRef {{
          target {{
            ... on Commit {{
              history {{ totalCount }}
              recentHistory: history(since: "{since}") {{ totalCount }}
            }}
          }}
        }}
      }}
    }}
    membersWithRole {{ totalCount }}
  }}
}}"#
    )
}

fn chrono_since_30d() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let thirty_days = 30 * 24 * 60 * 60;
    let since = now - thirty_days;
    // Format as ISO 8601
    let secs_per_day = 86400;
    let days_since_epoch = since / secs_per_day;

    // Simple date calculation
    let mut year = 1970i64;
    let mut remaining_days = days_since_epoch as i64;

    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let month_days = if is_leap(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for &days in &month_days {
        if remaining_days < days {
            break;
        }
        remaining_days -= days;
        month += 1;
    }
    let day = remaining_days + 1;

    format!("{year:04}-{month:02}-{day:02}T00:00:00Z")
}

fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
