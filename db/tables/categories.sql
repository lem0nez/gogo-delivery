-- Copyright © 2023 Nikita Dudko. All rights reserved.
-- Contacts: <nikita.dudko.95@gmail.com>
-- Licensed under the MIT License.

CREATE TABLE public.categories
(
    id serial NOT NULL,
    title character varying(128) NOT NULL,
    description text,
    preview bytea,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.categories
    OWNER to gogo;
