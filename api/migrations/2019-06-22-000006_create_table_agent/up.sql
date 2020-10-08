CREATE TABLE `agent`
(
    `id`   INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `name` VARCHAR(255)                            NOT NULL DEFAULT ''
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
