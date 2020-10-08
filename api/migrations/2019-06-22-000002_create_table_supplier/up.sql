CREATE TABLE `supplier`
(
    `id`               INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `telegram_login`   BIGINT                                  NOT NULL DEFAULT 0,
    `name`             VARCHAR(255)                            NOT NULL DEFAULT '',
    `about`            VARCHAR(1000)                           NOT NULL,
    `address`          VARCHAR(80)                             NOT NULL DEFAULT '',
    `lng`              FLOAT                                   NOT NULL DEFAULT 0,
    `lat`              FLOAT                                   NOT NULL DEFAULT 0,
    `poly_lng_min`     FLOAT                                   NOT NULL DEFAULT 0,
    `poly_lng_max`     FLOAT                                   NOT NULL DEFAULT 0,
    `poly_lat_min`     FLOAT                                   NOT NULL DEFAULT 0,
    `poly_lat_max`     FLOAT                                   NOT NULL DEFAULT 0,
    `status`           TINYINT UNSIGNED                        NOT NULL DEFAULT 2,
    `chat_id`          BIGINT                                  NOT NULL DEFAULT 0,
    `telegram_user_id` BIGINT                                  NOT NULL DEFAULT 0,
    `shift`            TINYINT UNSIGNED                        NOT NULL DEFAULT 0,
    `created_at`       TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`       TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX supplier_bounding (`poly_lng_min`, `poly_lng_max`, `poly_lat_min`, `poly_lat_max`),
    INDEX telegram_login (`telegram_login`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
