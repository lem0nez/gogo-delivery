SELECT
    id,
    title,
    -- Do not select 'preview' as it contains large data (JPEG image).
    description,
    count,
    is_alcohol,
    price
FROM
    food
WHERE
    category_id = $1;
