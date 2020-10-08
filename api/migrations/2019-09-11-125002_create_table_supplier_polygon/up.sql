CREATE TABLE `supplier_polygon`
(
    `supplier_id` INT UNSIGNED      NOT NULL,
    `sequence`    SMALLINT UNSIGNED NOT NULL,
    `lng`         FLOAT             NOT NULL,
    `lat`         FLOAT             NOT NULL,
    PRIMARY KEY (`supplier_id`, `sequence`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
