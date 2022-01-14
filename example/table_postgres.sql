CREATE TABLE biz_activity
(
    id            varchar(50)  NOT NULL DEFAULT '',
    name          varchar(255) NOT NULL,
    pc_link       varchar(255)          DEFAULT NULL,
    h5_link       varchar(255)          DEFAULT NULL,
    sort          varchar(255) NOT NULL,
    status        int          NOT NULL,
    version       int          NOT NULL,
    remark        varchar(255)          DEFAULT NULL,
    create_time   timestamp    NOT NULL,
    delete_flag   int          NOT NULL,
    pc_banner_img varchar(255)          DEFAULT NULL,
    h5_banner_img varchar(255)          DEFAULT NULL,
    PRIMARY KEY (id)
);