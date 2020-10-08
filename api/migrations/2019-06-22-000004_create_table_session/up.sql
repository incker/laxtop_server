CREATE TABLE session
(
    `hash`       VARCHAR(40) PRIMARY KEY NOT NULL,
    `owner_type` TINYINT UNSIGNED        NOT NULL,
    `owner_id`   INT UNSIGNED            NOT NULL,
    `expired_at` TIMESTAMP               NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
