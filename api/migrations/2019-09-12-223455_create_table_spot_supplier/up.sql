CREATE TABLE `spot_supplier`
(
    `spot_id`     INT UNSIGNED NOT NULL,
    `supplier_id` INT UNSIGNED NOT NULL,
    PRIMARY KEY (`spot_id`, `supplier_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

# именно в таком порядке потому что так чаще выборка делается
