-- Loaded by MariaDB after rAthena's own /sql-files/*.sql scripts.
-- File is named zz-admin.sql so it sorts AFTER main.sql, web.sql, etc.
--
-- Seeds a default GM admin account so the freshly built server is
-- immediately usable:
--   userid:    admin
--   password:  123
--   group_id:  99 (full GM, see conf/groups.yml)

USE `ragnarok`;

INSERT INTO `login`
    (`account_id`, `userid`, `user_pass`, `sex`, `email`, `group_id`)
VALUES
    (2000000, 'admin', '123', 'M', 'admin@rathena.local', 99)
ON DUPLICATE KEY UPDATE
    `userid`    = VALUES(`userid`),
    `user_pass` = VALUES(`user_pass`),
    `sex`       = VALUES(`sex`),
    `email`     = VALUES(`email`),
    `group_id`  = VALUES(`group_id`);
