CREATE TABLE `user_spot`
(
    `user_id` INT UNSIGNED NOT NULL,
    `spot_id` INT UNSIGNED NOT NULL,
    PRIMARY KEY (`user_id`, `spot_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
