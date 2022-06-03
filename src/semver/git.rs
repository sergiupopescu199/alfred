use anyhow::{Context, Result};
use git2::{DescribeFormatOptions, DescribeOptions, Repository};

pub fn get_latest_tag(repo_path: String) -> Result<String> {
    let mut current_version = "0.0.0".to_string();

    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let tags = repo
        .describe(DescribeOptions::default().describe_tags())
        .into_iter();

    let tags = tags
        .map(|a| {
            a.format(Some(DescribeFormatOptions::default().abbreviated_size(0)))
                .unwrap_or("0.0.0".to_string())
        })
        .collect::<Vec<_>>();

    if !tags.is_empty() {
        current_version = tags.into_iter().next().context("unable to get tag")?;
    }
    Ok(current_version)
}

pub fn update_version(repo_path: String, tag: String) -> Result<String> {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let obj = repo.revparse_single("HEAD")?;
    let sig = repo.signature()?;
    let f = repo.tag(&tag, &obj, &sig, &tag, true)?;

    Ok(f.to_string())
}
