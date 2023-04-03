use anyhow::Result;
use octocrab::{self, models::Repository};

pub async fn search_repositories(search_query: String) -> Result<Vec<Repository>> {
    octocrab::instance()
        .search()
        .repositories(&search_query)
        .per_page(20)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch github repos, {}", e))
        .map(|page| page.items)
}
