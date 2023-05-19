INSERT INTO orders_food
(
    order_id,
    food_id,
    count
)
VALUES ($1, $2, $3)
RETURNING id;
