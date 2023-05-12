/*
 *         Copyright (C) 2023. Ziyi Joshua LIU
 *         This program is free software: you can redistribute it and/or modify
 *         it under the terms of the GNU General Public License as published by
 *         the Free Software Foundation, either version 3 of the License, or
 *         (at your option) any later version.
 *
 *         This program is distributed in the hope that it will be useful,
 *         but WITHOUT ANY WARRANTY; without even the implied warranty of
 *         MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *         GNU General Public License for more details.
 *
 *         You should have received a copy of the GNU General Public License
 *         along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use std::path::PathBuf;
use git2::{Oid, Repository};
use log::{debug, error};

pub(crate) fn path2repo(path: String, output_path: PathBuf) -> Result<Repository, git2::Error> {
    // match path is a path buf or url
    let repo: Repository = match path.starts_with("http") {
        true => Repository::clone(path.as_str(), output_path.as_path()),
        false => Repository::open(path.as_str()),
    }?;
    Ok(repo)
}


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
                // let blob = object.as_blob().unwrap();
                // debug!("{}", str::from_utf8(blob.content()).unwrap());

                // Or you can print the path of the blob
                debug!("{}", entry.name().unwrap());
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