create table ld_user
(
    user_id       integer PRIMARY KEY,
    user_password text NOT NULL,
    user_nickname text,
    user_name     text,
    user_phone    integer,
    user_email    text,
    user_role     integer
)