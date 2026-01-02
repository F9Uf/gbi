use git2::{Branch, BranchType, Error, Repository};

fn map_branch_to_name_string(branch: Branch) -> Option<String> {
    branch.name()
        .ok()
        .flatten()
        .map(String::from)
}

fn valid_branch(branch_result: Result<(Branch<'_>, BranchType), Error>) -> Option<Branch> {
    branch_result
        .ok()
        .map(|(branch, _)| branch)
}

pub fn get_current_branch(repo_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_name)?;
    let head = repo.head()?;
    let shorthand = head.shorthand().ok_or("Invalid branch name")?;
    Ok(shorthand.to_string())
}

pub fn list_branches(repo_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_name)?;
    let branches_iter = repo.branches(None)?;

    let branch_names: Vec<String> = branches_iter
        .filter_map(|branch_result|
            valid_branch(branch_result)
            .and_then(map_branch_to_name_string)
        )
        .collect();

    Ok(branch_names)
}

pub fn checkout_branch(repo_name: &str, branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_name)?;
    let (object, reference) = repo.revparse_ext(branch_name)?;

    repo.checkout_tree(&object, None)?;

    match reference {
        Some(gref) => repo.set_head(gref.name().unwrap())?,
        None => repo.set_head_detached(object.id())?,
    }

    Ok(())
}

pub fn delete_branch(repo_name: &str, branch_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_name)?;
    let mut branch = repo.find_branch(branch_name, BranchType::Local)?;
    branch.delete()?;
    Ok(())
}
