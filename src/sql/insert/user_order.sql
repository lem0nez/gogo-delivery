INSERT INTO orders
(
    customer_id,
    address_id,
    create_time
)
VALUES
(
    $1,
    (
        SELECT
            id
        FROM
            addresses
        WHERE
            id = $2
        AND
            customer_id = $3
    ),
    CURRENT_TIMESTAMP
)
RETURNING id;
