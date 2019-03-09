DROP TABLE IF EXISTS `biz_activity`;
CREATE TABLE `biz_activity` (
  `id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `uuid` varchar(50) DEFAULT NULL,
  `pc_link` varchar(255) DEFAULT NULL,
  `h5_link` varchar(255) DEFAULT NULL,
  `remark` varchar(255) DEFAULT NULL,
  `version` int(11) DEFAULT 0,
  `create_time` datetime NULL,
  `delete_flag` int(1) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 ROW_FORMAT=DYNAMIC COMMENT='test';

-- ----------------------------
-- Records of biz_activity
-- ----------------------------
INSERT INTO `biz_activity` VALUES ('165', '安利一波大表哥', null, 'http://www.baidu.com', 'http://www.taobao.com', 'ceshi', '2018-05-23 15:21:22', '1');
INSERT INTO `biz_activity` VALUES ('166', '注册送好礼', null, '', 'www.baidu.com', '测试', '2018-05-24 10:36:31', '1');
INSERT INTO `biz_activity` VALUES ('167', 'hello', null, 'www.baidu.com', 'www.baidu.com', 'ceshi', '2018-05-24 10:41:17', '1');
INSERT INTO `biz_activity` VALUES ('168', 'rs168', null, null, null, null, '2018-05-23 15:21:22', '1');
INSERT INTO `biz_activity` VALUES ('169', 'rs168', null, null, null, null, '2018-05-23 15:21:22', '1');
INSERT INTO `biz_activity` VALUES ('170', 'rs168-10', null, null, null, null, '2018-05-23 15:21:22', '1');



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