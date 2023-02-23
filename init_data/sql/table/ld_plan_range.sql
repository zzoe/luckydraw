drop table ld_plan_range;
create table ld_plan_range
(
    act_id    integer not null
        constraint ld_plan_range_ld_activity_act_id_fk
            references ld_activity,
    act_seq   integer not null,
    cus_flag  TEXT,
    flag_type integer
);
