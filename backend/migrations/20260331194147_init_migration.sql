-- Add migration script here

CREATE TYPE tabletype AS ENUM('categories', 'account', 'operations');
CREATE TYPE action AS ENUM('create', 'update', 'delete');
CREATE TYPE emailcodeaction AS ENUM('change_password', 'change_email', 'register');

CREATE TABLE IF NOT EXISTS public.users
(
    email character varying(255) COLLATE pg_catalog."default" NOT NULL,
    password_hash character varying(255) COLLATE pg_catalog."default" NOT NULL,
    name character varying(100) COLLATE pg_catalog."default" NOT NULL,
    verified boolean NOT NULL,
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_email_key UNIQUE (email),
    CONSTRAINT users_id_key UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS public.accounts
(
    user_id uuid NOT NULL,
    name character varying(100) COLLATE pg_catalog."default" NOT NULL,
    funds numeric(10, 2) NOT NULL,
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    color character varying COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT accounts_pkey PRIMARY KEY (id),
    CONSTRAINT accounts_id_key UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS public.alembic_version
(
    version_num character varying(32) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT alembic_version_pkc PRIMARY KEY (version_num)
);

CREATE TABLE IF NOT EXISTS public.categories
(
    user_id uuid NOT NULL,
    name character varying(100) COLLATE pg_catalog."default" NOT NULL,
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    CONSTRAINT categories_pkey PRIMARY KEY (id),
    CONSTRAINT categories_id_key UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS public.email_codes
(
    code_hash character varying(255) COLLATE pg_catalog."default" NOT NULL,
    action emailcodeaction NOT NULL,
    user_id uuid NOT NULL,
    expires_at timestamp with time zone NOT NULL,
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    CONSTRAINT email_codes_pkey PRIMARY KEY (id),
    CONSTRAINT email_codes_id_key UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS public.operations
(
    account_uuid uuid NOT NULL,
    transfer_id uuid,
    amount numeric(10, 2) NOT NULL,
    date timestamp with time zone NOT NULL,
    notes character varying(1024) COLLATE pg_catalog."default" NOT NULL,
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    category_id uuid,
    CONSTRAINT operations_pkey PRIMARY KEY (id),
    CONSTRAINT operations_id_key UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS public.sync_operations
(
    processing_id uuid NOT NULL,
    action action NOT NULL,
    table_type tabletype NOT NULL,
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    CONSTRAINT sync_operations_pkey PRIMARY KEY (id),
    CONSTRAINT sync_operations_id_key UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS public.transfers
(
    account_from_id uuid NOT NULL,
    account_to_id uuid NOT NULL,
    id uuid NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    CONSTRAINT transfers_pkey PRIMARY KEY (id),
    CONSTRAINT transfers_id_key UNIQUE (id)
);

ALTER TABLE IF EXISTS public.accounts
    ADD CONSTRAINT accounts_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.categories
    ADD CONSTRAINT categories_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.email_codes
    ADD CONSTRAINT email_codes_user_id_fkey FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.operations
    ADD CONSTRAINT operations_account_uuid_fkey FOREIGN KEY (account_uuid)
    REFERENCES public.accounts (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.operations
    ADD CONSTRAINT operations_category_id_fkey FOREIGN KEY (category_id)
    REFERENCES public.categories (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.operations
    ADD CONSTRAINT operations_transfer_id_fkey FOREIGN KEY (transfer_id)
    REFERENCES public.transfers (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.transfers
    ADD CONSTRAINT transfers_account_from_id_fkey FOREIGN KEY (account_from_id)
    REFERENCES public.accounts (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.transfers
    ADD CONSTRAINT transfers_account_to_id_fkey FOREIGN KEY (account_to_id)
    REFERENCES public.accounts (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;