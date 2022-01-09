create table user_security
(
    id serial,
    user_id int not null
        constraint user_security_users_id_fk
            references users
            on update cascade on delete cascade,
    login_duration int default 7 not null,
    two_factor bool default false not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

create unique index user_security_id_uindex
    on user_security (id);

create unique index user_security_user_id_uindex
    on user_security (user_id);

alter table user_security
    add constraint user_security_pk
        primary key (id);

select diesel_manage_updated_at('user_security');
