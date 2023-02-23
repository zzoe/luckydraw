drop table ld_plan;
create table ld_plan
(
    act_id        integer not null
        constraint ld_plan_ld_activity_act_id_fk references ld_activity,
    act_seq       integer not null,
    act_prize     TEXT,
    prize_picture BLOB,
    prize_amount  integer
);

create unique index ld_plan_act_id_act_seq_uindex on ld_plan (act_id, act_seq);
