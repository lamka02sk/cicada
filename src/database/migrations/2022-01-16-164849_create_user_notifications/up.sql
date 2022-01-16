create table user_notifications
(
    id serial,
    user_id int not null
        constraint user_notifications_users_id_fk
            references users
            on update cascade on delete cascade,
    auth_login boolean default true not null,
    auth_password_change boolean default true not null,
    auth_attempt boolean default true not null,
    deploy_start boolean default true not null,
    deploy_finish boolean default true not null,
    deploy_fail boolean default true not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
);

create unique index user_notifications_id_uindex
    on user_notifications (id);

create unique index user_notifications_user_id_uindex
    on user_notifications (user_id);

alter table user_notifications
    add constraint user_notifications_pk
        primary key (id);

select diesel_manage_updated_at('user_notifications');