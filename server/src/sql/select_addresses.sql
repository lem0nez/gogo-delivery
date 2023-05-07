SELECT
    *
FROM
    addresses
WHERE
    customer_id = $1
ORDER BY
    -- Internal tuple ID signifying physical order.
    ctid
DESC;
