drop table ld_user;
create table ld_user
(
    user_id       integer primary key autoincrement,
    user_account  text not null unique,
    user_password text not null,
    user_nickname text,
    user_name     text,
    user_phone    integer,
    user_email    text,
    role_id       integer
);

