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
    food,
	orders_food
WHERE
    orders_food.order_id = $1
AND
    orders_food.food_id = food.id;
