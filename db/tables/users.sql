-- Copyright © 2023 Nikita Dudko. All rights reserved.
-- Contacts: <nikita.dudko.95@gmail.com>
-- Licensed under the MIT License.

CREATE TYPE "UserRole" AS ENUM
(
    'Customer',
    'Rider',
    'Manager'
);

CREATE TABLE public.users
(
    id serial NOT NULL,
    username character varying(64) NOT NULL,
    password character(64) NOT NULL,
    first_name character varying(128),
    last_name character varying(128),
    birth_date date NOT NULL,
    role "UserRole" NOT NULL DEFAULT 'Customer',
    PRIMARY KEY (id),
    CONSTRAINT username UNIQUE (username)
);

ALTER TABLE IF EXISTS public.users
    OWNER to gogo;
