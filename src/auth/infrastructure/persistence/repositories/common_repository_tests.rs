use crate::auth::domain::entities::user::User;
use crate::auth::domain::repositories::user_repository::IUserRepository;
use crate::shared::domain::value_objects::UserUuid;

#[cfg(test)]
pub async fn assert_user_repository_behavior(repo: Box<dyn IUserRepository>, user: User) {
    // Run modular tests
    test_save_and_get(repo.as_ref(), &user).await;
    test_get_non_existent(repo.as_ref()).await;
    test_find_by_email(repo.as_ref(), &user).await;
    test_find_by_email_non_existent(repo.as_ref()).await;
    test_save_duplicate_user(repo.as_ref(), &user).await;
}

async fn test_save_and_get(repo: &dyn IUserRepository, user: &User) {
    repo.save(user).await.expect("Should save user");

    let fetched = repo
        .get(user.id)
        .await
        .expect("Should not error on get")
        .expect("Should find saved user");

    assert_eq!(fetched.id, user.id);
    assert_eq!(fetched.email.value(), user.email.value());
}

async fn test_get_non_existent(repo: &dyn IUserRepository) {
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

async fn test_find_by_email(repo: &dyn IUserRepository, user: &User) {
    let fetched_by_email = repo
        .find_by_email(user.clone().email)
        .await
        .expect("Should not error on find by email")
        .expect("Should find saved user by email");

    assert_eq!(fetched_by_email.id, user.id);
    assert_eq!(fetched_by_email.email.value(), user.email.value());
}

async fn test_find_by_email_non_existent(repo: &dyn IUserRepository) {
    use crate::auth::domain::entities::user::UserEmail;
    let non_existent_email = UserEmail::new("nonexistent@example.com").unwrap();
    let result = repo
        .find_by_email(non_existent_email)
        .await
        .expect("Should not error on non-existent find by email");
    assert!(
        result.is_none(),
        "Should return None for non-existent user by email, not an error"
    );
}

async fn test_save_duplicate_user(repo: &dyn IUserRepository, user: &User) {
    let result = repo.save(user).await;
    assert!(
        result.is_err(),
        "Should return error when saving user with same ID"
    );

    // Test duplicate email with different ID
    let new_user_id = UserUuid::new().to_string();
    let duplicate_email_user = User::new(&new_user_id, user.email.value(), "AnotherP@ssw0rd!")
        .expect("Should create user");

    let result_email = repo.save(&duplicate_email_user).await;
    assert!(
        result_email.is_err(),
        "Should return error when saving user with duplicate email"
    );
}
