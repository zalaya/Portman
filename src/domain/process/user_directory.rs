use sysinfo::{ Uid, Users };

pub struct UserDirectory(Users);

impl UserDirectory {
    pub fn snapshot() -> Self {
        Self(Users::new_with_refreshed_list())
    }

    pub(super) fn name_for(&self, uid: &Uid) -> Option<String> {
        self.0.get_user_by_id(uid).map(|user| user.name().to_string())
    }
}
