-- Copyright © 2023 Nikita Dudko. All rights reserved.
-- Contacts: <nikita.dudko.95@gmail.com>
-- Licensed under the MIT License.

CREATE ROLE gogo WITH
	LOGIN
	NOSUPERUSER
	NOCREATEDB
	NOCREATEROLE
	INHERIT
	NOREPLICATION
	CONNECTION LIMIT -1
	PASSWORD 'gogo_delivery';
