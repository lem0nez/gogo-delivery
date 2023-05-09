INSERT INTO categories
(
    title,
    description,
    preview
)
VALUES ($1, $2, $3)
RETURNING id;
