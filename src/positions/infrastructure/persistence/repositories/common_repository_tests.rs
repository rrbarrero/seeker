use crate::positions::domain::entities::position::{Position, PositionUuid};
use crate::positions::domain::repositories::position_repository::IPositionRepository;

#[cfg(test)]
pub async fn assert_repository_behavior(
    mut repo: Box<dyn IPositionRepository>,
    position: Position,
) {
    let position_id = position.id;

    // 1. Test save and get
    repo.save(position.clone())
        .await
        .expect("Should save position");

    let fetched = repo
        .get(position_id)
        .await
        .expect("Should not error on get")
        .expect("Should find saved position");

    assert_eq!(fetched.id, position_id);
    assert!(!fetched.is_deleted());

    // 2. Test get_all
    let all = repo.get_all().await.expect("Should get all positions");
    assert!(all.iter().any(|p| p.id == position_id));

    // 3. Test remove (soft delete)
    repo.remove(position_id)
        .await
        .expect("Should remove position");

    let deleted_position = repo
        .get(position_id)
        .await
        .expect("Should not error on get after remove")
        .expect("Should still find position after soft delete");

    assert!(deleted_position.is_deleted());
    assert!(deleted_position.deleted_at.is_some());

    // 4. Test getting non-existent position
    let non_existent_id = PositionUuid::new();
    let result = repo
        .get(non_existent_id)
        .await
        .expect("Should not error on non-existent get");
    assert!(
        result.is_none(),
        "Should return None for non-existent position, not an error"
    );

    // 5. Test removing non-existent position (should be idempotent or return a consistent error)
    // We'll decide on idempotency (Ok(())) for now as it's common in repos.
    let result = repo.remove(non_existent_id).await;
    assert!(
        result.is_ok(),
        "Remove should be idempotent and return Ok even if not found"
    );
}
