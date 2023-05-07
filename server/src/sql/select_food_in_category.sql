SELECT
    id,
    title,
    description,
    -- Do not select 'preview' as it contains large data (JPEG image).
    category_id,
    count,
    is_alcohol,
    price
FROM
    food
WHERE
    category_id = $1;
