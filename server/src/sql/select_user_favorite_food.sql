SELECT
    food.id,
    food.title,
    food.description,
    -- Do not select 'preview' as it contains large data (JPEG image).
    food.category_id,
    food.count,
    food.is_alcohol,
    food.price
FROM
    favorites,
    food
WHERE
    favorites.user_id = $1
AND
    favorites.food_id = food.id;
