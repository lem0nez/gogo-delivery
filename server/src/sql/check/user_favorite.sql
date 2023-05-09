SELECT EXISTS
(
    SELECT
        1
    FROM
        favorites
    WHERE
    	user_id = $1
	AND
		food_id = $2
);
