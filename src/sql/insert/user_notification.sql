INSERT INTO notifications
(
    user_id,
    sent_time,
    title,
    description
)
VALUES
(
    $1,
    CURRENT_TIMESTAMP,
    $2,
    $3
)
RETURNING id;
