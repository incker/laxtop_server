CREATE TABLE `contact_phone`
(
    `id`           INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `number`       VARCHAR(20)                             NOT NULL DEFAULT '',
    `country_code` VARCHAR(2)                              NOT NULL DEFAULT '',
    `name`         VARCHAR(255)                            NOT NULL DEFAULT '',
    `rank`         VARCHAR(255)                            NOT NULL DEFAULT '',
    `status`       TINYINT UNSIGNED                        NOT NULL DEFAULT 2,
    `created_at`   TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`   TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;
