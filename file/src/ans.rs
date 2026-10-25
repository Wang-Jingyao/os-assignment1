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
        Self {
            inner: RefCell::new(Inner {
                latest_read_time: SystemTime::now(),
                permitted_users: HashSet::new(),
            }),
            data: r_str.as_bytes().to_vec(),
            owner: owner.to_string(),
        }
    }

    /// Reads the file and does permission check. Only the owner and permitted users can read this file.
    ///
    /// If the permission check is passed, update the `latest_read_time` and read the data.
    pub fn read(&self, user: &str) -> Result<&[u8], Error> {
        let mut inner = self.inner.borrow_mut();

        if user != self.owner && !inner.permitted_users.contains(user) {
            return Err(Error::PermissionDenied);
        }

        inner.latest_read_time = SystemTime::now();
        Ok(self.data.as_slice())
    }

    /// Returns the owner of this file.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the latest read time.
    pub fn latest_read_time(&self) -> SystemTime {
        self.inner.borrow().latest_read_time
    }

    /// Returns the permitted users. Owner is not included.
    pub fn permitted_users(&self) -> HashSet<String> {
        self.inner.borrow().permitted_users.clone()
    }

    /// Adds permitted user. Only owner can add permitted user.
    ///
    /// Do nothing and return `Ok` if the user is the owner.
    pub fn add_permitted_user(&self, current_user: &str, user: &str) -> Result<(), Error> {
        if current_user != self.owner {
            return Err(Error::PermissionDenied);
        }

        if user == self.owner {
            return Ok(());
        }

        self.inner.borrow_mut().permitted_users.insert(user.to_string());
        Ok(())
    }

    /// Removes permitted user. Only owner can remove permitted user.
    ///
    /// If the removed permitted user is the owner, return `PermissionDenied`.
    pub fn remove_permitted_user(&self, current_user: &str, user: &str) -> Result<(), Error> {
        if current_user != self.owner || user == self.owner {
            return Err(Error::PermissionDenied);
        }

        self.inner.borrow_mut().permitted_users.remove(user);
        Ok(())
    }
}
