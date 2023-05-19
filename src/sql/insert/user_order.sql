INSERT INTO orders
(
    customer_id,
    address_id,
    create_time
)
VALUES
(
    $1,
    $2,
    CURRENT_TIMESTAMP
)
RETURNING id;
