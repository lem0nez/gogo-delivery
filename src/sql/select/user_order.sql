SELECT
    *
FROM
    orders
WHERE
    customer_id = $1
AND
    id = $2;
