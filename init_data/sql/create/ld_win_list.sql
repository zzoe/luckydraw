drop table ld_win_list;
create table ld_win_list
(
    act_id  integer not null
        constraint ld_win_list_ld_activity_act_id_fk references ld_activity,
    act_seq integer not null,
    cus_id  integer not null
);