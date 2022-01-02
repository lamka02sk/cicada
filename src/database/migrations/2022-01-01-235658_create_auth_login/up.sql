create table auth_login
(
    id         serial,
    uuid       uuid      default gen_random_uuid() not null,
    user_id    int                                 not null
        constraint auth_login_users_id_fk
            references users
            on update cascade on delete cascade,
    secret     varchar(128)                        not null,
    token      varchar(128)                        not null,
    user_agent varchar(256)                        not null,
    ip_address inet                                not null,
    active     bool      default true              not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

create index auth_login_active_index
    on auth_login (active desc);

create unique index auth_login_id_uindex
    on auth_login (id);

create index auth_login_user_id_index
    on auth_login (user_id);

create unique index auth_login_uuid_uindex
    on auth_login (uuid);

alter table auth_login
    add constraint auth_login_pk
        primary key (id);
