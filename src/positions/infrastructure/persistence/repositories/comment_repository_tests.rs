use crate::positions::domain::entities::comment::{Comment, CommentUuid};
use crate::positions::domain::entities::position::PositionUuid;
use crate::positions::domain::repositories::comment_repository::ICommentRepository;

#[cfg(test)]
pub async fn assert_repository_behavior(repo: Box<dyn ICommentRepository>, comment: Comment) {
    let comment_id = comment.id;
    let position_id = comment.position_id;

    // 1. Test save and get
    repo.save(comment.clone())
        .await
        .expect("Should save comment");

    let fetched = repo
        .get(comment_id)
        .await
        .expect("Should not error on get")
        .expect("Should find saved comment");

    assert_eq!(fetched.id, comment_id);
    assert_eq!(fetched.position_id, position_id);

    // 2. Test get_by_position
    let by_position = repo
        .get_by_position(position_id)
        .await
        .expect("Should get comments by position");
    assert!(by_position.iter().any(|c| c.id == comment_id));

    // 3. Test update
    let mut updated = fetched.clone();
    updated.body = crate::positions::domain::entities::comment::CommentBody::new("Updated");
    repo.update(updated.clone())
        .await
        .expect("Should update comment");

    let fetched_updated = repo
        .get(comment_id)
        .await
        .expect("Should not error on get")
        .expect("Should find updated comment");

    assert_eq!(fetched_updated.body.value(), "Updated");

    // 4. Test remove
    repo.remove(comment_id)
        .await
        .expect("Should remove comment");

    let deleted = repo
        .get(comment_id)
        .await
        .expect("Should not error on get after remove");

    assert!(deleted.is_none());

    // 5. Test getting non-existent comment
    let non_existent_id = CommentUuid::new();
    let result = repo
        .get(non_existent_id)
        .await
        .expect("Should not error on non-existent get");
    assert!(
        result.is_none(),
        "Should return None for non-existent comment, not an error"
    );

    // 6. Test getting by position with no comments
    let other_position = PositionUuid::new();
    let empty = repo
        .get_by_position(other_position)
        .await
        .expect("Should return empty list");
    assert!(empty.is_empty());
}
