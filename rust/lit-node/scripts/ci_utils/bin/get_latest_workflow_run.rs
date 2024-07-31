use log::{debug, info};
use serde::Deserialize;
use utils::get_target_branch;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Get the name of the workflow from the 1st command line argument.
    let workflow_name = std::env::args().nth(1).expect("No workflow name provided");
    // Get the name of the target branch from the 2nd command line argument.
    let target_branch = std::env::args().nth(2).expect("No target branch provided");
    let target_branch = get_target_branch(&target_branch).expect("Failed to get target branch");

    let mut workflow_list = vec![];
    let mut page = 1;
    let mut done = false;
    let per_page = 100;
    while !done {
        // Fetch the workflow from the REST API.
        let req = populate_github_api_headers(
        reqwest::Client::new()
            .get(format!("https://api.github.com/repos/lit-protocol/lit-assets/actions/workflows?per_page={}&page={}", per_page, page)),
        )
        .send()
        .await
        .expect("Failed to fetch Github workflows");

        // Deserialize the response into a workflow list.
        let workflow_page: WorkflowList = req
            .json()
            .await
            .expect("Failed to deserialize workflow list");
        info!("Workflow list page {}: {:?}", page, workflow_page);
        workflow_list.extend(workflow_page.workflows);
        page += 1;
        if page * per_page >= workflow_page.total_count {
            done = true;
        }
    }

    // Find the workflow with the given name.
    let workflow = workflow_list
        .iter()
        .find(|w| w.name == workflow_name)
        .expect("No workflow found with the given name");

    let mut workflow_run_list = vec![];
    let mut page = 1;
    let mut done = false;
    let per_page = 100;
    while !done {
        // Fetch the workflow runs from the REST API.
        let url = format!(
        "https://api.github.com/repos/lit-protocol/lit-assets/actions/workflows/{}/runs?branch={}&per_page={}&page={}",
        workflow.id, target_branch, per_page, page
    );
        debug!("Fetching workflow runs from: {}", url);
        let req = populate_github_api_headers(reqwest::Client::new().get(url))
            .send()
            .await
            .expect("Failed to fetch Github workflow runs");
        let res_body_str = req.text().await.expect("Failed to get response body");
        debug!("Response body: {}", res_body_str);

        // Deserialize the response into a workflow run list.
        let workflow_run_page: WorkflowRunList =
            serde_json::from_str(&res_body_str).expect("Failed to deserialize workflow run list");
        info!("Workflow run list page {}: {:?}", page, workflow_run_page);
        workflow_run_list.extend(workflow_run_page.workflow_runs);
        page += 1;
        if page * per_page >= workflow_run_page.total_count {
            done = true;
        }
    }

    // Get the first workflow run from the list.
    let workflow_run = workflow_run_list.first().expect("No workflow run found");

    // Print the ID of the first workflow run.
    println!("{}", workflow_run.id);
}

fn populate_github_api_headers(req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    req.header("Accept", "application/vnd.github.v4+json")
        .header(
            "Authorization",
            format!("Bearer {}", std::env::var("GH_PAT").unwrap()),
        )
        .header("X-Github-Api-Version", "2022-11-28")
        .header("User-Agent", "lit-protocol-downloader")
}

#[derive(Debug, Deserialize)]
struct WorkflowList {
    total_count: u64,
    workflows: Vec<Workflow>,
}

#[derive(Debug, Deserialize)]
struct Workflow {
    id: u64,
    node_id: String,
    name: String,
    path: String,
    state: String,
    created_at: String,
    updated_at: String,
    url: String,
    html_url: String,
    badge_url: String,
}

#[derive(Debug, Deserialize)]
struct WorkflowRunList {
    total_count: u64,
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    id: u64,
    node_id: String,
    head_branch: String,
    head_sha: String,
    run_number: u64,
    event: String,
    status: String,
    conclusion: String,
    workflow_id: u64,
    url: String,
    html_url: String,
    created_at: String,
    updated_at: String,
    jobs_url: String,
    logs_url: String,
    check_suite_url: String,
    artifacts_url: String,
    cancel_url: String,
    rerun_url: String,
    workflow_url: String,
}
