INSERT INTO feedbacks
(
    order_id,
    rating,
    comment
)
VALUES ($1, $2, $3)
RETURNING id;
