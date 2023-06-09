CREATE TABLE public.orders
(
    id serial NOT NULL,
    customer_id serial NOT NULL,
    address_id serial NOT NULL,
    create_time timestamp without time zone NOT NULL,
    rider_id integer,
    completed_time timestamp without time zone,
    PRIMARY KEY (id),
    CONSTRAINT customer_id FOREIGN KEY (customer_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT address_id FOREIGN KEY (address_id)
        REFERENCES public.addresses (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE RESTRICT
        NOT VALID,
    CONSTRAINT rider_id FOREIGN KEY (rider_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE SET NULL
        NOT VALID
);

ALTER TABLE IF EXISTS public.orders
    OWNER to gogo;
