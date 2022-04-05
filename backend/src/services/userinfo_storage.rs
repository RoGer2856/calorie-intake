pub type UserInfo = crate::services::AuthorizationInfo;

pub trait UserInfoStorage: Send {
    fn update_userinfo(&mut self, userinfo: UserInfo) -> Result<(), UserInfoStorageError>;
    fn iter_userinfo<'a>(
        &'a mut self,
    ) -> Result<Box<dyn Iterator<Item = UserInfo> + 'a>, UserInfoStorageError>;
}

#[derive(Debug, PartialEq)]
pub enum UserInfoStorageError {
    InternalError,
}
