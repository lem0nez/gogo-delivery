CREATE TABLE public.cart
(
    id serial NOT NULL,
    customer_id serial NOT NULL,
    food_id serial NOT NULL,
    count integer NOT NULL DEFAULT 1,
    add_time timestamp without time zone NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT customer_id FOREIGN KEY (customer_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT food_id FOREIGN KEY (food_id)
        REFERENCES public.food (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT count CHECK (count > 0) NOT VALID
);

ALTER TABLE IF EXISTS public.cart
    OWNER to gogo;
