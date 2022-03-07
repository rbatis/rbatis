create table biz_activity
(
    id   uuid not null constraint biz_activity_pk primary key,
    name        varchar,
    create_time timestamp,
    js          json
);