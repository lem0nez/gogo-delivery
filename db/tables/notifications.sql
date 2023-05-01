CREATE TABLE public.notifications
(
    id serial NOT NULL,
    user_id serial NOT NULL,
    sent_time timestamp without time zone NOT NULL,
    title character varying(128) NOT NULL,
    description text,
    PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

ALTER TABLE IF EXISTS public.notifications
    OWNER to gogo;
