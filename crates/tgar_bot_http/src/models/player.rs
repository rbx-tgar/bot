use tgar_bot_utils::serde_struct;

serde_struct!(ExpOperation {
    /// the id of the player this operation is being performed on
    id: i64,
    /// the xp amount for this operation
    xp: i64
});
