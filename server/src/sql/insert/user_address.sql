INSERT INTO addresses
(
    customer_id,
    locality,
    street,
    house,
    corps,
    apartment
)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING id;
