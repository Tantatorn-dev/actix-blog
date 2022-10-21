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

CREATE TABLE public.blog (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	blog_user_id int4 NULL,
	title text NULL,
	"content" text NULL,
	CONSTRAINT fk_blog_user FOREIGN KEY (blog_user_id) REFERENCES public.blog_user(id)
);

CREATE TABLE public.user_session (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	blog_user_id int4 NULL,
	"token" text NULL DEFAULT md5(random()::text),
	CONSTRAINT fk_blog_user FOREIGN KEY (blog_user_id) REFERENCES public.blog_user(id)
);
