use std::str;

use git2::{Oid, Repository};
use log::{debug, error};

// Recursively traverse tree objects
fn traverse_tree(repo: &Repository, oid: Oid) -> Result<(), git2::Error> {
    // Find the tree object by its OID
    let tree = repo.find_tree(oid)?;

    // Traverse each entry of the tree object
    for entry in tree.iter() {
        // Convert the entry to a Git object
        let object = entry.to_object(&repo)?;

        // Perform the corresponding operation according to the object type
        match object.kind() {
            Some(git2::ObjectType::Blob) => {
                // If the object is a blob, perform the corresponding operation
                // For example, you can print the content of the blob
                let blob = object.as_blob().unwrap();
                debug!("{}", str::from_utf8(blob.content()).unwrap());
            }
            Some(git2::ObjectType::Tree) => {
                // If the object is a tree, recursively traverse its content
                let tree_oid = object.as_tree().unwrap().id();
                traverse_tree(repo, tree_oid)?;
            }
            _ => {
                // If the object is not a blob or a tree, ignore it
                error!("Unknown Git object type");
            }
        }
    }

    Ok(())
}

// Traverse the commit that the HEAD reference points to
pub(crate) fn traverse_head(repo: &Repository) -> Result<(), git2::Error> {
    // Get the HEAD reference
    let head = repo.head()?;
    // Get the OID of the commit object
    let oid = head.peel_to_commit()?.tree_id();
    // Recursively traverse the tree object
    traverse_tree(repo, oid)?;
    Ok(())
}