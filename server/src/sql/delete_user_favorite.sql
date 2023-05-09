DELETE FROM
    favorites
WHERE
    user_id = $1
AND
    id = $2;
