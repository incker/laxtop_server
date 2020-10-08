CREATE TABLE `supplier_sequence`
(
    `user_id`     INT UNSIGNED      NOT NULL,
    `spot_id`     INT UNSIGNED      NOT NULL,
    `supplier_id` INT UNSIGNED      NOT NULL,
    `sequence`    SMALLINT UNSIGNED NOT NULL DEFAULT 0,
    PRIMARY KEY (`user_id`, `spot_id`, `supplier_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
