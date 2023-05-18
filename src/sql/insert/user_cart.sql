INSERT INTO cart
(
    customer_id,
    food_id,
    count,
    add_time
)
VALUES
(
    $1,
    $2,
    $3,
    CURRENT_TIMESTAMP
);
