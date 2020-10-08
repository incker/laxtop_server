CREATE TABLE `user`
(
    `id`               INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `number`           VARCHAR(255)                            NOT NULL DEFAULT '',
    `country_code`     VARCHAR(2)                              NOT NULL DEFAULT '',
    `name`             VARCHAR(255)                            NOT NULL DEFAULT '',
    `license_accepted` TIMESTAMP                               NOT NULL DEFAULT '2000-01-01 00:00:00',
    `status`           TINYINT UNSIGNED                        NOT NULL DEFAULT 2,
    `creator_type`     TINYINT UNSIGNED                        NOT NULL,
    `creator_id`       INT UNSIGNED                            NOT NULL,
    `created_at`       TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`       TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY (`number`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
