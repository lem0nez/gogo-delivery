-- Copyright © 2023 Nikita Dudko. All rights reserved.
-- Contacts: <nikita.dudko.95@gmail.com>
-- Licensed under the MIT License.

CREATE TABLE public.addresses
(
    id serial NOT NULL,
    customer_id serial NOT NULL,
    locality character varying(128) NOT NULL,
    street character varying(128) NOT NULL,
    house integer NOT NULL,
    corps character varying(16),
    apartment character varying(16),
    PRIMARY KEY (id),
    CONSTRAINT customer_id FOREIGN KEY (customer_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

ALTER TABLE IF EXISTS public.addresses
    OWNER to gogo;
