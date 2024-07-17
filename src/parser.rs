use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    pub name: Option<String>,
    pub on: On,
    pub env: Option<HashMap<String, String>>,
    pub defaults: Option<Defaults>,
    pub concurrency: Option<Concurrency>,
    pub jobs: HashMap<String, Job>,
    pub permissions: Option<Permissions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum On {
    Simple(String),
    Multiple(Vec<String>),
    Complex(HashMap<String, EventConfig>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventConfig {
    pub types: Option<Vec<String>>,
    pub branches: Option<Vec<String>>,
    #[serde(rename = "branches-ignore")]
    pub branches_ignore: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "tags-ignore")]
    pub tags_ignore: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    #[serde(rename = "paths-ignore")]
    pub paths_ignore: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Defaults {
    pub run: Option<RunDefaults>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunDefaults {
    pub shell: Option<String>,
    #[serde(rename = "working-directory")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Concurrency {
    Simple(String),
    Complex(ConcurrencyConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    pub group: String,
    #[serde(rename = "cancel-in-progress")]
    pub cancel_in_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub name: Option<String>,
    pub needs: Option<Vec<String>>,
    #[serde(rename = "runs-on")]
    pub runs_on: RunsOn,
    pub permissions: Option<Permissions>,
    pub env: Option<HashMap<String, String>>,
    pub defaults: Option<Defaults>,
    #[serde(rename = "if")]
    pub if_condition: Option<String>,
    pub steps: Vec<Step>,
    #[serde(rename = "timeout-minutes")]
    pub timeout_minutes: Option<u32>,
    pub strategy: Option<Strategy>,
    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<bool>,
    pub container: Option<Container>,
    pub services: Option<HashMap<String, Container>>,
    #[serde(rename = "concurrency")]
    pub job_concurrency: Option<Concurrency>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunsOn {
    Single(String),
    Multiple(Vec<String>),
    Matrix(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub actions: Option<String>,
    pub checks: Option<String>,
    pub contents: Option<String>,
    pub deployments: Option<String>,
    #[serde(rename = "id-token")]
    pub id_token: Option<String>,
    pub issues: Option<String>,
    pub packages: Option<String>,
    #[serde(rename = "pull-requests")]
    pub pull_requests: Option<String>,
    #[serde(rename = "repository-projects")]
    pub repository_projects: Option<String>,
    #[serde(rename = "security-events")]
    pub security_events: Option<String>,
    pub statuses: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "if")]
    pub if_condition: Option<String>,
    pub uses: Option<String>,
    pub run: Option<String>,
    #[serde(rename = "working-directory")]
    pub working_directory: Option<String>,
    pub shell: Option<String>,
    pub with: Option<HashMap<String, String>>,
    pub env: Option<HashMap<String, String>>,
    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<bool>,
    #[serde(rename = "timeout-minutes")]
    pub timeout_minutes: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Strategy {
    pub matrix: Option<serde_yml::Value>,
    #[serde(rename = "fail-fast")]
    pub fail_fast: Option<bool>,
    #[serde(rename = "max-parallel")]
    pub max_parallel: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    pub image: String,
    pub credentials: Option<Credentials>,
    pub env: Option<HashMap<String, String>>,
    pub ports: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
    pub options: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
