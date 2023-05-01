CREATE TABLE public.orders_food
(
    id serial NOT NULL,
    order_id serial NOT NULL,
    food_id serial NOT NULL,
    count integer NOT NULL DEFAULT 1,
    PRIMARY KEY (id),
    CONSTRAINT order_id FOREIGN KEY (order_id)
        REFERENCES public.orders (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT food_id FOREIGN KEY (food_id)
        REFERENCES public.food (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CHECK (count > 0) NOT VALID
);

ALTER TABLE IF EXISTS public.orders_food
    OWNER to gogo;
