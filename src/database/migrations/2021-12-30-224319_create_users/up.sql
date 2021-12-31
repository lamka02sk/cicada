create table users
(
    id         serial,
    uuid       uuid      default gen_random_uuid() not null,
    firstname  varchar(64)                         not null,
    lastname   varchar(64)                         not null,
    email      varchar(128)                        not null,
    password   varchar(64)                         not null,
    token      varchar(128)                        not null,
    enabled    boolean   default false             not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

create unique index users_email_uindex
    on users (email);

create unique index users_id_uindex
    on users (id);

create unique index users_token_uindex
    on users (token);

create unique index users_uuid_uindex
    on users (uuid);

alter table users
    add constraint users_pk
        primary key (id);
