DROP TABLE IF EXISTS `biz_activity`;
CREATE TABLE `biz_activity` (
  `id` varchar(50) NOT NULL DEFAULT '' COMMENT '唯一活动码',
  `name` varchar(255) NOT NULL,
  `pc_link` varchar(255) DEFAULT NULL,
  `h5_link` varchar(255) DEFAULT NULL,
  `sort` varchar(255) NOT NULL COMMENT '排序',
  `status` int(11) NOT NULL COMMENT '状态（0：已下线，1：已上线）',
  `version` int(11) NOT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `create_time` datetime NOT NULL,
  `delete_flag` int(1) NOT NULL,
  `pc_banner_img` varchar(255) DEFAULT NULL,
  `h5_banner_img` varchar(255) DEFAULT NULL,
  `bbb` bit(4) DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 ROW_FORMAT=COMPACT COMMENT='运营管理-活动管理';

-- ----------------------------
-- Records of biz_activity
-- ----------------------------
INSERT INTO `test`.`biz_activity` (`id`, `name`, `sort`, `status`, `version`, `create_time`, `delete_flag`) VALUES ('1', '活动1', '1', '1', '1', '2019-12-12 00:00:00', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('11bdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享18', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-26 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('12bdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享17', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-25 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('13bdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享16', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-24 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('14bdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享15', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-23 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('15bdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享14', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-22 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('16bdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享13', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-21 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('17bdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享12', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-20 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('1fbdd779-5f70-4b8f-9921-a235a9c75b69', '新人专享11', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '6', '备注一下', '2019-05-19 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('2fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享10', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-18 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('3fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享9', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-17 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('4fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享8', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-16 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('5fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享7', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-15 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('6fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享6', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-14 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('7fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享5', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-13 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('8fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享4', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-12 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('9fbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享3', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-11 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('dfbdd779-5f70-4b8f-9921-a235a9c75b69', '新人专享2', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '6', '备注一下', '2019-05-10 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');
INSERT INTO `test`.`biz_activity` (`id`, `name`, `pc_link`, `h5_link`, `sort`, `status`, `version`, `remark`, `create_time`, `delete_flag`, `pc_banner_img`, `h5_banner_img`, `bbb`) VALUES ('dfbdd779-5f70-4b8f-9921-c235a9c75b69', '新人专享1', 'http://115.220.9.139:8002/newuser/', 'http://115.220.9.139:8002/newuser/', '0', '0', '7', '备注一下', '2019-05-09 10:25:41', '1', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', 'http://47.110.8.203:8080/group1/default/20190527/10/25/0/新人专享banner.jpg?download=0', '1');



DROP TABLE IF EXISTS `biz_property`;
CREATE TABLE `biz_property` (
  `id` varchar(255) NOT NULL,
  `user_id` varchar(255) NOT NULL,
  `total_amount` decimal(18,0) DEFAULT '0',
  `total_profit` decimal(18,0) DEFAULT '0',
  `can_use_amount` decimal(18,0) DEFAULT '0' COMMENT '平台可用余额',
  `freeze` decimal(18,0) DEFAULT '0' COMMENT '冻结',
  `pool_total_amount` decimal(18,0) DEFAULT '0' COMMENT '资金池，总金额（余额+收益）',
  `pool_amount` decimal(18,0) DEFAULT '0' COMMENT '资金池/钱包余额',
  `pool_profit` decimal(18,0) DEFAULT '0' COMMENT '资金池收益',
  `pool_freeze` decimal(18,0) DEFAULT '0' COMMENT '资金池投资冻结',
  `accumulation_profit` decimal(18,0) DEFAULT '0' COMMENT '累计收益',
  `integral` decimal(18,0) DEFAULT '0' COMMENT '积分',
  `create_time` datetime DEFAULT NULL,
  `update_time` datetime DEFAULT NULL,
  `status` int(11) NOT NULL DEFAULT '1',
  `delete_flag` int(1) DEFAULT '1',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 ROW_FORMAT=DYNAMIC;


INSERT INTO `test`.`biz_property` (`id`, `user_id`, `total_amount`, `total_profit`, `can_use_amount`, `freeze`, `pool_total_amount`, `pool_amount`, `pool_profit`, `pool_freeze`, `accumulation_profit`, `integral`, `create_time`, `update_time`, `status`, `delete_flag`) VALUES ('20180926172014a2a48a9491004603', '20180926172013b85403d3715d46ed', '14500', '0', '0', '0', '14500', '14800', '0', '100', '0', '0', '2018-09-26 17:20:14', '2018-10-23 16:34:21', '0', '1');
INSERT INTO `test`.`biz_property` (`id`, `user_id`, `total_amount`, `total_profit`, `can_use_amount`, `freeze`, `pool_total_amount`, `pool_amount`, `pool_profit`, `pool_freeze`, `accumulation_profit`, `integral`, `create_time`, `update_time`, `status`, `delete_flag`) VALUES ('2018102316263216637761fee373d4', '20181023162632152fd236d6877ff4', '14500', '0', '0', '0', '14500', '14400', '0', '0', '0', '0', '2018-10-23 16:26:32', '2018-10-23 16:35:00', '1', '1');