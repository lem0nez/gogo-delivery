SELECT
    id,
    title,
    description
    -- Do not select 'preview' as it contains large data (JPEG image).
FROM
    categories
ORDER BY
    title;
