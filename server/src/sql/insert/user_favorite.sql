INSERT INTO favorites
(
    user_id,
    food_id,
    add_time
)
VALUES
(
    $1,
    $2,
    CURRENT_TIMESTAMP
)
RETURNING id;
