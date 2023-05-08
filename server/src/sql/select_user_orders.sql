SELECT
    *
FROM
    orders
WHERE
    customer_id = $1
ORDER BY
    create_time
DESC;
