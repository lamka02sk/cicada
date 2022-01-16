alter table auth_login
    add two_factor_code varchar(6) default '000000' not null;
