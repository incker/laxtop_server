CREATE TABLE `admin`
(
    `id`     INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `name`   VARCHAR(255)                            NOT NULL DEFAULT '',
    `status` TINYINT UNSIGNED                        NOT NULL DEFAULT 2
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;
