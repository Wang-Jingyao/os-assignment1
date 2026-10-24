use std::{cell::RefCell, collections::HashSet, time::SystemTime};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    PermissionDenied,
}

pub struct ReadOnlyFile {
    inner: RefCell<Inner>,
    data: Vec<u8>,
    owner: String,
}

struct Inner {
    latest_read_time: SystemTime,
    permitted_users: HashSet<String>,
}

impl ReadOnlyFile {
    /// Constructs self from `&str`.
    pub fn from_str(r_str: &str, owner: &str) -> Self {
        todo!()
    }

    /// Reads the file and does permission check. Only the owner and permitted users can read this file.
    ///
    /// If the permission check is passed, update the `latest_read_time` and read the data.
    pub fn read(&self, user: &str) -> Result<&[u8], Error> {
        todo!()
    }

    /// Returns the owner of this file.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the latest read time.
    pub fn latest_read_time(&self) -> SystemTime {
        todo!()
    }

    /// Returns the permitted users. Owner is not included.
    pub fn permitted_users(&self) -> HashSet<String> {
        todo!()
    }

    /// Adds permitted user. Only owner can add permitted user.
    ///
    /// Do nothing and return `Ok` if the user is the owner.
    pub fn add_permitted_user(&self, current_user: &str, user: &str) -> Result<(), Error> {
        todo!()
    }

    /// Removes permitted user. Only owner can remove permitted user.
    ///
    /// If the removed permitted user is the owner, return `PermissionDenied`.
    pub fn remove_permitted_user(&self, current_user: &str, user: &str) -> Result<(), Error> {
        todo!()
    }
}
