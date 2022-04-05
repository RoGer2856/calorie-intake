use super::*;

pub struct InMemoryUserInfoStorage {
    userinfos: Vec<UserInfo>,
}

pub struct InMemoryUserInfoStorageIterator<'a> {
    iter: std::slice::Iter<'a, UserInfo>,
}

impl InMemoryUserInfoStorage {
    pub fn new() -> Self {
        Self {
            userinfos: Vec::new(),
        }
    }
}

impl UserInfoStorage for InMemoryUserInfoStorage {
    fn update_userinfo(&mut self, userinfo: UserInfo) -> Result<(), UserInfoStorageError> {
        if let Some(info) = self
            .userinfos
            .iter_mut()
            .find(|item| item.username == userinfo.username)
        {
            info.role = userinfo.role;
            info.max_calories_per_day = userinfo.max_calories_per_day;
        } else {
            self.userinfos.push(userinfo);
        }

        Ok(())
    }

    fn iter_userinfo<'a>(
        &'a mut self,
    ) -> Result<Box<dyn Iterator<Item = UserInfo> + 'a>, UserInfoStorageError> {
        Ok(Box::new(InMemoryUserInfoStorageIterator::new(
            self.userinfos.iter(),
        )))
    }
}

impl<'a> InMemoryUserInfoStorageIterator<'a> {
    pub fn new(iter: std::slice::Iter<'a, UserInfo>) -> Self {
        Self { iter }
    }
}

impl<'a> Iterator for InMemoryUserInfoStorageIterator<'a> {
    type Item = UserInfo;

    fn next(&mut self) -> Option<UserInfo> {
        self.iter.next().map(|userinfo_ref| userinfo_ref.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_userinfo() {
        let userinfo_jane = UserInfo {
            username: "jane".into(),
            role: RoleType::RegularUser,
            max_calories_per_day: 2100,
        };

        let userinfo_john = UserInfo {
            username: "john".into(),
            role: RoleType::RegularUser,
            max_calories_per_day: 2100,
        };

        let mut userinfo_storage = InMemoryUserInfoStorage::new();

        userinfo_storage
            .update_userinfo(userinfo_jane.clone())
            .unwrap();
        userinfo_storage
            .update_userinfo(userinfo_john.clone())
            .unwrap();

        let userinfos: Vec<UserInfo> = userinfo_storage.iter_userinfo().unwrap().collect();
        assert_eq!(2, userinfos.len());

        userinfos
            .iter()
            .find(|userinfo| userinfo_jane == **userinfo)
            .unwrap();
        userinfos
            .iter()
            .find(|userinfo| userinfo_john == **userinfo)
            .unwrap();

        let userinfo_john_modified = UserInfo {
            username: "john".into(),
            role: RoleType::Admin,
            max_calories_per_day: 1900,
        };
        userinfo_storage
            .update_userinfo(userinfo_john_modified.clone())
            .unwrap();

        let userinfos: Vec<UserInfo> = userinfo_storage.iter_userinfo().unwrap().collect();
        assert_eq!(2, userinfos.len());

        userinfos
            .iter()
            .find(|userinfo| userinfo_jane == **userinfo)
            .unwrap();
        userinfos
            .iter()
            .find(|userinfo| userinfo_john_modified == **userinfo)
            .unwrap();
    }
}
