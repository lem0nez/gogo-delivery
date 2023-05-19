UPDATE
    orders
SET
    rider_id = $1
WHERE
    id = $2
AND
    rider_id IS NULL;
