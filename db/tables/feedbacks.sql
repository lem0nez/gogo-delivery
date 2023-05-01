-- Copyright © 2023 Nikita Dudko. All rights reserved.
-- Contacts: <nikita.dudko.95@gmail.com>
-- Licensed under the MIT License.

CREATE TABLE public.feedbacks
(
    id serial NOT NULL,
    order_id serial NOT NULL,
    rating smallint,
    comment text,
    PRIMARY KEY (id),
    CONSTRAINT order_id FOREIGN KEY (order_id)
        REFERENCES public.orders (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT rating CHECK (rating >= 0 AND rating <= 5) NOT VALID
);

ALTER TABLE IF EXISTS public.feedbacks
    OWNER to gogo;
