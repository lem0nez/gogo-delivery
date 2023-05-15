SELECT EXISTS
(
    SELECT
        1
    FROM
        cart
    WHERE
    	customer_id = $1
	AND
		food_id = $2
);
