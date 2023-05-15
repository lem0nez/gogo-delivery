SELECT
    *
FROM
    orders_food
WHERE
    order_id = $1
ORDER BY
    -- Internal tuple ID signifying physical order.
    ctid
DESC;
