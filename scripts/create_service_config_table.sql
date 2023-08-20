-- filename: create_service_config_table.sql

CREATE TABLE service_config (
    `id` INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键',
    `service` VARCHAR(255) NOT NULL COMMENT '服务名称',
    `key` VARCHAR(255) NOT NULL COMMENT '配置键',
    `value` TEXT COMMENT '配置值',
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间'
);
