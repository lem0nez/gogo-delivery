INSERT INTO users
(
    username,
    password,
    first_name,
    last_name,
    birth_date
)
VALUES ($1, $2, $3, $4, $5)
RETURNING id;
