DELETE FROM
    cart
WHERE
    customer_id = $1
AND
    id = $2;
