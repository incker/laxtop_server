CREATE TABLE `invoice_product`
(
    `invoice_id` INT UNSIGNED NOT NULL,
    `product_id` INT UNSIGNED NOT NULL,
    `amount`     INT UNSIGNED NOT NULL,
    PRIMARY KEY (`invoice_id`, `product_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
