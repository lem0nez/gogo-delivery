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
