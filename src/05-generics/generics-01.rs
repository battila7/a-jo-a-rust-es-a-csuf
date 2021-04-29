trait UserRepository {
    fn find_user_by_id(&self, id: String) -> Option<String>;
}

fn is_user_exists(id: String, repository: UserRepository) -> bool {
    repository.find_user_by_id(id).is_some()
}
