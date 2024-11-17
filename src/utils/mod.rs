/// This module contains utility functions that are used throughout the project.
///

/// Returns the git hash of the current commit.
///
/// # Returns
/// The git hash of the current commit.
pub fn githash<'a>() -> &'a str {
    env!("GIT_HASH")
}

/// Returns the git tag of the current commit.
///
/// # Returns
/// The git tag of the current commit.
pub fn gittag<'a>() -> &'a str {
    env!("GIT_TAG")
}
