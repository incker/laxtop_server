CREATE TABLE `promo`
(
    `id`          INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `supplier_id` INT UNSIGNED                            NOT NULL,
    `cat_id`      INT UNSIGNED                            NOT NULL,
    `image_id`    INT UNSIGNED                            NOT NULL,
    `created_at`  TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
