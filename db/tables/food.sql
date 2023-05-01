-- Copyright © 2023 Nikita Dudko. All rights reserved.
-- Contacts: <nikita.dudko.95@gmail.com>
-- Licensed under the MIT License.

CREATE TABLE public.food
(
    id serial NOT NULL,
    title character varying(128) NOT NULL,
    description text,
    preview bytea,
    category_id serial NOT NULL,
    count integer NOT NULL DEFAULT 0,
    is_alcohol boolean NOT NULL,
    price_byn numeric(7, 2) NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT category_id FOREIGN KEY (category_id)
        REFERENCES public.categories (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE RESTRICT
        NOT VALID
);

ALTER TABLE IF EXISTS public.food
    OWNER to gogo;
