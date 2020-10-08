CREATE TABLE `promo_group`
(
    `id`   INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `name` VARCHAR(255)                            NOT NULL DEFAULT ''
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

INSERT IGNORE INTO `promo_group` (`id`, `name`)
VALUES ('1', 'Без группы');
