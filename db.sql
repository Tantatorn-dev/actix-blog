-- public.blog_user definition

-- Drop table

-- DROP TABLE public.blog_user;

CREATE TABLE public.blog_user (
	id int4 NOT NULL DEFAULT nextval('blog_user_id_seq'::regclass),
	username varchar(50) NOT NULL,
	"password" varchar(50) NOT NULL,
	CONSTRAINT accounts_pkey PRIMARY KEY (id),
	CONSTRAINT accounts_username_key UNIQUE (username)
);