drop table ld_role;
create table ld_role
(
    role_id   integer not null
    constraint ld_role_pk primary key autoincrement,
    role_name TEXT
);

