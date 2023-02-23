drop table ld_user_role;
create table ld_user_role
(
    role_id      integer not null,
    role_type    integer,
    privilege_id integer
);