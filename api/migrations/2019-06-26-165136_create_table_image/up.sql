CREATE TABLE image
(
    `id`    INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `dir`   VARCHAR(2)                              NOT NULL,
    `dir2`  VARCHAR(2)                              NOT NULL,
    `hash`  VARCHAR(10)                             NOT NULL,
    `hash2` VARCHAR(20)                             NOT NULL,
    INDEX (`hash`),
    UNIQUE KEY (`dir`, `dir2`, `hash`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

# all images are webp, so no need to write down extension

# id 1 image is for test spot
INSERT IGNORE INTO `image` (`id`, `dir`, `dir2`, `hash`, `hash2`)
VALUES ('1', 'aa', 'aa', 'test_spot', '');
