drop table ld_custom;
create table ld_custom
(
    cus_id       integer not null
        constraint ld_custom_pk primary key autoincrement,
    cus_nickname TEXT    not null,
    cus_name     TEXT,
    cus_picture  BLOB,
    cus_phone    integer,
    cus_identity TEXT,
    cus_flag     TEXT
);

create unique index ld_custom_cus_nickname_uindex
    on ld_custom (cus_nickname);

