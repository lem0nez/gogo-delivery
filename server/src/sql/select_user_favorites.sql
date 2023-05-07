SELECT
    *
FROM
    favorites
WHERE
    user_id = $1
ORDER BY
    add_time
DESC;
