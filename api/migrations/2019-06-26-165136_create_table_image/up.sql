CREATE TABLE image
(
    `id` INT UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

# all images are jpg, so no need to write down extension

# id 1 image is for test spot
INSERT IGNORE INTO `image` (`id`)
VALUES ('1');
