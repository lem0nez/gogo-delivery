DELETE FROM
    orders
WHERE
    customer_id = $1
AND
    id = $2
AND
    rider_id IS NULL;
