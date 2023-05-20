DELETE FROM
    addresses
WHERE
    customer_id = $1
AND
    id = $2;
