INSERT INTO ld_menu (menu_id, parent_id, menu_type, menu_name, menu_desc, page_id, menu_status) VALUES (1, 0, 0, '系统管理', '系统管理一级标签', 0, 1);
INSERT INTO ld_menu (menu_id, parent_id, menu_type, menu_name, menu_desc, page_id, menu_status) VALUES (2, 1, 1, '用户权限', null, 0, 1);
INSERT INTO ld_menu (menu_id, parent_id, menu_type, menu_name, menu_desc, page_id, menu_status) VALUES (3, 2, 2, '用户管理', null, 1001, 1);
INSERT INTO ld_menu (menu_id, parent_id, menu_type, menu_name, menu_desc, page_id, menu_status) VALUES (4, 2, 2, '角色管理', null, 1002, 1);
INSERT INTO ld_menu (menu_id, parent_id, menu_type, menu_name, menu_desc, page_id, menu_status) VALUES (5, 0, 0, '抽奖系统', '抽奖配置一级标签', 0, 1);
INSERT INTO ld_menu (menu_id, parent_id, menu_type, menu_name, menu_desc, page_id, menu_status) VALUES (6, 5, 1, '抽奖配置管理', '', 0, 1);
INSERT INTO ld_menu (menu_id, parent_id, menu_type, menu_name, menu_desc, page_id, menu_status) VALUES (7, 6, 2, '活动管理', null, 1003, 1);
