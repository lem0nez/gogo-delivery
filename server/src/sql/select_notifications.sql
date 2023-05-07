SELECT
    *
FROM
    notifications
WHERE
    user_id = $1
ORDER BY
    sent_time
DESC;
