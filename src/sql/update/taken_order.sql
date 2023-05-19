UPDATE
    orders
SET
    completed_time = CURRENT_TIMESTAMP
WHERE
    id = $1
AND
    rider_id = $2;
