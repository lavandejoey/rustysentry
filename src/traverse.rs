use git2::{Repository, BranchType};

fn traverse_repo(repo_path: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?;
    let branch = head.shorthand().unwrap_or("master");

    println!("Traversing repository {} (branch: {})", repo_path, branch);

    // Traverse commits
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;

        // Traverse tree
        let tree = commit.tree()?;
        traverse_tree(&repo, &tree)?;
    }
    return Ok(());
}

fn traverse_tree(repo: &Repository, tree: &git2::Tree) -> Result<(), git2::Error> {
    for entry in tree {
        let entry = entry?;
        let path = entry.name().unwrap_or("");

        if entry.filemode().is_file() {
            // Do something with file
            let blob = entry.to_object(repo)?.as_blob().unwrap();
            let content = std::str::from_utf8(blob.content())?;
            println!("Found file: {}", path);
            println!("Content: {}", content);
        } else if entry.filemode().is_tree() {
            // Recursively traverse subdirectory
            let sub_tree = entry.to_object(repo)?.as_tree().unwrap();
            traverse_tree(repo, &sub_tree)?;
        }
    }
    return Ok(());
}
