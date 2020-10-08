CREATE TABLE `product`
(
    `id`          INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `name`        VARCHAR(255)                            NOT NULL,
    `supplier_id` INT UNSIGNED                            NOT NULL,
    `key`         VARCHAR(100)                            NOT NULL DEFAULT '',
    `unit`        TINYINT UNSIGNED                        NOT NULL DEFAULT 0,
    `is_deleted`  TINYINT(1)                              NOT NULL DEFAULT 0,
    UNIQUE KEY (`name`, `supplier_id`, `key`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
