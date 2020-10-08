CREATE TABLE `spot`
(
    `id`           INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `address`      VARCHAR(80)                             NOT NULL DEFAULT '',
    `spot_type`    VARCHAR(80)                             NOT NULL DEFAULT '',
    `spot_name`    VARCHAR(255)                            NOT NULL DEFAULT '',
    `image_id`     INT UNSIGNED                            NOT NULL DEFAULT 0,
    `about`        TEXT                                    NOT NULL,
    `status`       TINYINT UNSIGNED                        NOT NULL DEFAULT 1,
    `country_code` VARCHAR(2)                              NOT NULL DEFAULT '',
    `lng`          FLOAT                                   NOT NULL,
    `lat`          FLOAT                                   NOT NULL,
    `creator_type` TINYINT UNSIGNED                        NOT NULL,
    `creator_id`   INT UNSIGNED                            NOT NULL,
    `created_at`   TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`   TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX location (`lng`, `lat`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

# id 1 spot is Test spot
INSERT IGNORE INTO `spot` (`id`, `address`, `spot_type`, `spot_name`, `image_id`,
                           `about`, `status`, `lng`, `lat`, `creator_type`, `creator_id`, `created_at`)
VALUES ('1', 'ул. Не существующая 0', 'Киоск', 'Шаурмун', '1', '',
        '1', '0.0', '0.0', '1', '1', CURRENT_TIMESTAMP);
