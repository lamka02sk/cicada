create table auth_attempts
(
    id         serial,
    uuid       uuid      default gen_random_uuid() not null,
    user_id    int                                 not null
        constraint auth_attempts_users_id_fk
            references users
            on update cascade on delete cascade,
    user_agent varchar(256)                        not null,
    ip_address inet                                not null,
    created_at timestamp default current_timestamp not null
);

create unique index auth_attempts_id_uindex
    on auth_attempts (id);

create index auth_attempts_user_id_index
    on auth_attempts (user_id);

create unique index auth_attempts_uuid_uindex
    on auth_attempts (uuid);

alter table auth_attempts
    add constraint auth_attempts_pk
        primary key (id);
