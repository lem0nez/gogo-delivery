CREATE TABLE public.favorites
(
    id serial NOT NULL,
    user_id serial NOT NULL,
    food_id serial NOT NULL,
    add_time timestamp without time zone NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT food_id FOREIGN KEY (food_id)
        REFERENCES public.food (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT food_per_user UNIQUE (user_id, food_id)
);

ALTER TABLE IF EXISTS public.favorites
    OWNER to gogo;
