trait UserRepository {
    fn find_user_by_id(&self, id: String) -> Option<String>;
}

trait UserService
{
    fn is_user_exists(&self, id: String) -> bool;
}

#[allow(dead_code)]
struct StaticDispatchUserService<R>
where
    R: UserRepository
{
    repository: R
}

impl<R> UserService for StaticDispatchUserService<R>
where
    R: UserRepository
{
    fn is_user_exists(&self, id: String) -> bool {
        self.repository.find_user_by_id(id).is_some()
    }
}
