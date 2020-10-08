CREATE TABLE `login`
(
    `owner_id`   INT UNSIGNED     NOT NULL,
    `owner_type` TINYINT UNSIGNED NOT NULL,
    `lg`         VARCHAR(255)     NOT NULL,
    `ps`         VARCHAR(255)     NOT NULL DEFAULT '',
    PRIMARY KEY (`owner_id`, `owner_type`),
    UNIQUE KEY (`owner_type`, `lg`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
