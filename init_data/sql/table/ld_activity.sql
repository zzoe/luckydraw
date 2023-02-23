drop table ld_activity;
create table ld_activity
(
    act_id          integer not null
        constraint ld_activity_pk primary key autoincrement,
    act_name        TEXT,
    act_picture     BLOB,
    act_description TEXT
);