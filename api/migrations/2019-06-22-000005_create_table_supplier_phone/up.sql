CREATE TABLE `supplier_phone`
(
    `supplier_id` INT UNSIGNED      NOT NULL,
    `sequence`    SMALLINT UNSIGNED NOT NULL DEFAULT 0,
    `position`    VARCHAR(255)      NOT NULL DEFAULT '',
    `number`      VARCHAR(255)      NOT NULL DEFAULT '',
    PRIMARY KEY (`supplier_id`, `sequence`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
