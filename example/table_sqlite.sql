CREATE TABLE `biz_activity`
(
    `id`            TEXT PRIMARY KEY NOT NULL,
    `name`          TEXT             NOT NULL,
    `pc_link`       TEXT DEFAULT NULL,
    `h5_link`       TEXT DEFAULT NULL,
    `sort`          TEXT             NOT NULL,
    `status`        INT              NOT NULL,
    `version`       INT              NOT NULL,
    `remark`        TEXT DEFAULT NULL,
    `create_time`   datetime         NOT NULL,
    `delete_flag`   INT(1)           NOT NULL,
    `pc_banner_img` TEXT DEFAULT NULL,
    `h5_banner_img` TEXT DEFAULT NULL
);

INSERT INTO `biz_activity`
VALUES ('1', '活动1', NULL, NULL, '1', 1, 1, 'fff', '2019-12-12 00:00:00', 0, NULL, NULL),
       ('178', 'test_insret', '', '', '1', 1, 0, '', '2020-06-17 20:08:13', 0, NULL, NULL),
       ('221', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL),
       ('222', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL),
       ('223', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL);
