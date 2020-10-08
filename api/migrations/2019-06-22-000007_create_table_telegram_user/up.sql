CREATE TABLE `telegram_user`
(
    # If the chat is a group, the chat id is negative. If it is a single person, then positive (any way better use i64 for bot compatibility)
    `id`         BIGINT PRIMARY KEY NOT NULL,
    `username`   VARCHAR(255)       NOT NULL DEFAULT '',
    `first_name` VARCHAR(255)       NOT NULL DEFAULT '',
    `last_name`  VARCHAR(255)       NOT NULL DEFAULT '',
    `photo_url`  VARCHAR(255)       NOT NULL DEFAULT '',
    `status`     TINYINT UNSIGNED   NOT NULL DEFAULT 2,
    `created_at` TIMESTAMP          NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP          NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
