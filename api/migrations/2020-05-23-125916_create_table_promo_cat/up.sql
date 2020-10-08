CREATE TABLE `promo_cat`
(
    `id`       INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `group_id` INT UNSIGNED                            NOT NULL DEFAULT 1,
    `name`     VARCHAR(255)                            NOT NULL DEFAULT ''
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
