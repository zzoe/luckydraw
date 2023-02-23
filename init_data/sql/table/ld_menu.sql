drop table ld_menu;
create table ld_menu
(
    menu_id     integer not null
        constraint ld_menu_pk primary key autoincrement,
    parent_id   integer,
    menu_type   integer,
    menu_name   TEXT,
    menu_desc   TEXT,
    page_id     integer,
    menu_status integer
);

