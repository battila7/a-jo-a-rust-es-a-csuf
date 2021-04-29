trait UserRepository {
    fn find_user_by_id(&self, id: String) -> Option<String>;
}

#[allow(dead_code)]
fn is_user_exists<T>(id: String, repository: &T) -> bool
where
    T: UserRepository
{
    repository.find_user_by_id(id).is_some()
}
