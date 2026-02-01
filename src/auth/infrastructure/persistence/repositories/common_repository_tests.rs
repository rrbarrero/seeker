use crate::auth::domain::entities::user::User;
use crate::auth::domain::repositories::user_repository::IUserRepository;
use crate::shared::domain::value_objects::UserUuid;

#[cfg(test)]
pub async fn assert_user_repository_behavior(repo: Box<dyn IUserRepository>, user: User) {
    let user_id = user.id;

    // 1. Test save and get
    repo.save(&user).await.expect("Should save user");

    let fetched = repo
        .get(user_id)
        .await
        .expect("Should not error on get")
        .expect("Should find saved user");

    assert_eq!(fetched.id, user_id);
    assert_eq!(fetched.email.value(), user.email.value());

    // 2. Test getting non-existent user
    let non_existent_id = UserUuid::new();
    let result = repo
        .get(non_existent_id)
        .await
        .expect("Should not error on non-existent get");
    assert!(
        result.is_none(),
        "Should return None for non-existent user, not an error"
    );
}
