INSERT INTO food
(
    title,
    description,
    preview,
    category_id,
    count,
    is_alcohol,
    price
)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id;
