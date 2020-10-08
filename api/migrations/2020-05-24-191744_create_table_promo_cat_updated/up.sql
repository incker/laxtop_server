CREATE TABLE `promo_cat_updated`
(
    `id`         INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `updated_at` TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

INSERT IGNORE INTO `promo_cat_updated` (`id`)
VALUES ('1');
