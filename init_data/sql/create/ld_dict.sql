drop table ld_dict;
create table ld_dict
(
    dict_key        integer not null
        constraint ld_dict_pk primary key autoincrement,
    dict_key_view   TEXT,
    dict_value      TEXT,
    dict_value_view TEXT,
    key_group       integer,
    value_note      TEXT
);

