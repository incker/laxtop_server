CREATE TABLE `user_promo_cat`
(
    `user_id`      INT UNSIGNED NOT NULL,
    `promo_cat_id` INT UNSIGNED NOT NULL,
    PRIMARY KEY (`user_id`, `promo_cat_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;