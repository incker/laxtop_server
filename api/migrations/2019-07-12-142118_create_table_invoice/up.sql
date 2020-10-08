CREATE TABLE `invoice`
(
    `id`             INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `creation_id`    INT UNSIGNED                            NOT NULL,
    `supplier_id`    INT UNSIGNED                            NOT NULL,
    `user_id`        INT UNSIGNED                            NOT NULL,
    `spot_id`        INT UNSIGNED                            NOT NULL,
    `position_count` TINYINT UNSIGNED                        NOT NULL,
    `status`         TINYINT UNSIGNED                        NOT NULL DEFAULT 2,
    `updated_at`     TIMESTAMP                               NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY (`creation_id`, `supplier_id`),
    UNIQUE KEY (`creation_id`, `user_id`),
    UNIQUE KEY (`creation_id`, `spot_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

# не делай разбивку на года чтоб не превратить в долгострой
# не делай любых других долгострой костылей
