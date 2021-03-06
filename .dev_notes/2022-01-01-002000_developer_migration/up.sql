# !If you have edited this file, maybe you will need to copy it to '.dev_notes/2019-09-23-002000_developer_migration'
# So you cold push it into git

# for fictitious supplier session
INSERT INTO `session` (`hash`, `owner_type`, `owner_id`)
VALUES ('gfasdfgdyjk78o689olymked56u24yhqefygws5r', '2', '1'),
       ('gfasdgdyjk78o689olymked56u24yhqefywked56', '1', '1');



INSERT INTO `supplier_phone` (`supplier_id`, `sequence`, `position`, `number`)
VALUES ('1', '1', 'Директор', '+30951970331'),
       ('1', '2', 'Прием заявок', '+30681325100');


# user for developing
INSERT INTO `user` (`id`, `name`, `status`, `creator_type`, `creator_id`, `created_at`)
VALUES ('1', 'Stanislav', '1', '1', '1', CURRENT_TIMESTAMP);


# spot for developing
INSERT INTO `spot` (`address`, `spot_type`, `spot_name`, `image_id`,
                    `about`, `status`, `lng`, `lat`, `creator_type`, `creator_id`, `created_at`)
VALUES ('ул. Поносенко 8', 'Киоск', 'Шаурмун', '0', 'Вьезд со двора\r\nработаем круглосуточно',
        '2', '0.0', '0.0', '1', '1', CURRENT_TIMESTAMP);


# supplier for developing
INSERT INTO `supplier` (`id`, `telegram_login`, `name`, `about`, `address`, `lng`, `lat`, `status`, `chat_id`, `shift`)
VALUES ('1', '482231043', 'Сигма', 'Мы самая крутая компания на рынке', 'ул. Поносенко 1',
        '0.00000', '0.00000', '1', '482231043', '0'),
       ('2', '0', 'Линиц', 'Мы самая крутая компания на рынке', 'ул. Поносенко 1',
        '0.00000', '0.00000', '1', '482231043', '0');


INSERT INTO `login` (`owner_id`, `owner_type`, `lg`, `ps`)
VALUES ('1', '2', '111', '111'),
       ('2', '2', '222', '222'),
       # agent
       ('1', '4', '123', '123');


# supplier products for developing
INSERT IGNORE INTO `product` (`name`, `supplier_id`, `key`, `unit`, `is_deleted`)
VALUES ('Вимикач', 1, '', 1, 0),
       ('Вимикач з підсвіткою', 1, '', 1, 0),
       ('Вимикач подвійний', 1, '', 1, 0),
       ('Вимикач подвійний з підсвітленням', 1, '', 1, 0),
       ('Вимикач прохідний', 1, '', 1, 0),
       ('Вимикач прохідний з підсвіткою', 1, '', 1, 0),
       ('Вимикач прохідний подвійний', 1, '', 1, 0),
       ('Кнопка дзвінка', 1, '', 1, 0),
       ('Кнопка дзвінка з підсвіткою', 1, '', 1, 0),
       ('Вимикач потрійний', 1, '', 1, 0),
       ('Розетка', 1, '', 1, 0),
       ('Розетка подвійна', 1, '', 1, 0),
       ('Розетка з заземленням', 1, '', 1, 0),
       ('Розетка подвійна з заземленням', 1, '', 1, 0),
       ('Розетка з заземленням та кришкою', 1, '', 1, 0),
       ('Розетка телевізійна', 1, '', 1, 0),
       ('Розетка телефонна', 1, '', 1, 0),
       ('Розетка телефонна подвійна', 1, '', 1, 0),
       ('Регулятор яскравості світла', 1, '', 1, 0),
       ('Розетка компьютерна', 1, '', 1, 0),
       ('Розетка компьютерна подвійна', 1, '', 1, 0),
       ('2-на рамка горизонтальна', 1, '', 1, 0),
       ('3-на рамка горизонтальна', 1, '', 1, 0),
       ('4-на рамка горизонтальна', 1, '', 1, 0),
       ('5-на рамка горизонтальна', 1, '', 1, 0),
       ('6-на рамка горизонтальна', 1, '', 1, 0);


# supplier products for developing
INSERT IGNORE INTO `product` (`name`, `supplier_id`, `key`, `unit`, `is_deleted`)
VALUES ('Редуктор 90*220/d150', 2, '', 1, 0),
       ('Редуктор 90*220/d150 (ярлик)', 2, '', 1, 0),
       ('913 З\'єднувач каналів 90*220/d150', 2, '', 1, 0),
       ('913 З\'єднувач каналів 90*220/d150 (ярлик)', 2, '', 1, 0),
       ('919 З\'єднувач каналів 90*220', 2, '', 1, 0),
       ('919 З\'єднувач каналів 90*220 (ярлик)', 2, '', 1, 0),
       ('9191 З\'єднувач каналів 90*220 з клапаном', 2, '', 1, 0),
       ('922 Коліно 90*220/d125', 2, '', 1, 0),
       ('923 Коліно 90*220/d150', 2, '', 1, 0),
       ('923 Коліно 90*220/d150 (ярлик)', 2, '', 1, 0),
       ('9291 Коліно горизонтальне 90*220', 2, '', 1, 0),
       ('9291 Коліно горизонтальне 90*220 (ярлик)', 2, '', 1, 0),
       ('9292 Коліно вертикальне 90*220', 2, '', 1, 0),
       ('9292 Коліно вертикальне 90*220 (ярлик)', 2, '', 1, 0),
       ('939 Трійник 90*220', 2, '', 1, 0),
       ('96 Утримувач 90*220', 2, '', 1, 0),
       ('96 Утримувач 90*220 (з/п)', 2, '', 1, 0),
       ('96 Утримувач 90*220 (ярлик)', 2, '', 1, 0),
       ('96М (з/п)', 2, '', 1, 0),
       ('96М Утримувач 90*220 (ярлик)', 2, '', 1, 0),
       ('З\'єднувач d100 для ГК', 2, '', 1, 0),
       ('З\'єднувач d100 для ГК (ярлик)', 2, '', 1, 0),
       ('З\'єднувач d125 для ГК', 2, '', 1, 0),
       ('З\'єднувач d125 для ГК (ярлик)', 2, '', 1, 0),
       ('З\'єднувач d150 для ГК', 2, '', 1, 0),
       ('З\'єднувач d150 для ГК (ярлик)', 2, '', 1, 0),
       ('З\'єднувач d200 для ГК', 2, '', 1, 0),
       ('З\'єднувач 55*110 для ГК', 2, '', 1, 0),
       ('З\'єднувач 55*110 для ГК (т/п)', 2, '', 1, 0),
       ('З\'єднувач 55*110 для ГК (ярлик)', 2, '', 1, 0),
       ('З\'єднувач 60х120 для ГК', 2, '', 1, 0),
       ('З\'єднувач 60х120 для ГК (ярлык)', 2, '', 1, 0),
       ('З\'єднувач 204х 60 для ГК', 2, '', 1, 0),
       ('З\'єднувач 204х 60 для ГК (ярлик)', 2, '', 1, 0),
       ('Спіровент 100/1', 2, '', 1, 0),
       ('Спіровент 100/2', 2, '', 1, 0),
       ('Спіровент 100/3', 2, '', 1, 0),
       ('Спіровент 1000/1', 2, '', 1, 0),
       ('Спіровент 1000/2', 2, '', 1, 0),
       ('Спіровент 1120/1', 2, '', 1, 0),
       ('Спіровент 1120/2', 2, '', 1, 0),
       ('Спіровент 125/1', 2, '', 1, 0),
       ('Спіровент 125/2', 2, '', 1, 0),
       ('Спіровент 125/3', 2, '', 1, 0),
       ('Спіровент 140/1', 2, '', 1, 0),
       ('Спіровент 140/2', 2, '', 1, 0),
       ('Спіровент 150/1', 2, '', 1, 0),
       ('Спіровент 150/2', 2, '', 1, 0),
       ('Спіровент 150/3', 2, '', 1, 0),
       ('Спіровент 160/1', 2, '', 1, 0),
       ('Спіровент 160/2', 2, '', 1, 0),
       ('Спіровент 160/3', 2, '', 1, 0),
       ('Спіровент 180/1', 2, '', 1, 0),
       ('Спіровент 180/2', 2, '', 1, 0),
       ('Спіровент 200/1', 2, '', 1, 0),
       ('Спіровент 200/2', 2, '', 1, 0),
       ('Спіровент 200/3', 2, '', 1, 0),
       ('Спіровент 224/1', 2, '', 1, 0),
       ('Спіровент 224/2', 2, '', 1, 0),
       ('Спіровент 250/1', 2, '', 1, 0),
       ('Спіровент 250/2', 2, '', 1, 0),
       ('Спіровент 250/3', 2, '', 1, 0),
       ('Спіровент 280/1', 2, '', 1, 0),
       ('Спіровент 280/2', 2, '', 1, 0),
       ('Спіровент 315/1', 2, '', 1, 0),
       ('Спіровент 315/2', 2, '', 1, 0),
       ('Спіровент 315/3', 2, '', 1, 0),
       ('Спіровент 355/1', 2, '', 1, 0),
       ('Спіровент 355/2', 2, '', 1, 0),
       ('Спіровент 355/3', 2, '', 1, 0),
       ('Спіровент 400/1', 2, '', 1, 0),
       ('Спіровент 400/2', 2, '', 1, 0),
       ('Спіровент 400/3', 2, '', 1, 0),
       ('Спіровент 450/1', 2, '', 1, 0),
       ('Спіровент 450/2', 2, '', 1, 0),
       ('Спіровент 450/3', 2, '', 1, 0),
       ('Врізка 125/125', 2, '', 1, 0),
       ('Врізка 150/125', 2, '', 1, 0),
       ('Врізка 150/150', 2, '', 1, 0),
       ('Врізка 160/125', 2, '', 1, 0),
       ('Врізка 160/150', 2, '', 1, 0),
       ('Врізка 160/160', 2, '', 1, 0),
       ('Врізка 200/125', 2, '', 1, 0),
       ('Врізка 200/140', 2, '', 1, 0),
       ('Врізка 200/150', 2, '', 1, 0),
       ('Врізка 200/160', 2, '', 1, 0),
       ('Врізка 200/200', 2, '', 1, 0);



INSERT INTO `spot_supplier` (`spot_id`, `supplier_id`)
VALUES ('1', '1'),
       ('1', '2');
INSERT INTO `user_spot` (`user_id`, `spot_id`)
VALUES ('1', '1');



INSERT INTO `promo_cat` (`id`, `group_id`, `name`)
VALUES (1, 1, 'Пиво вкусное'),
       (2, 1, 'Пиво по-вкуснее'),
       (3, 1, 'Пиво еще вкуснее'),
       (4, 1, 'Пиво Вкуснейшее'),
       (5, 2, 'Сидр вкусный'),
       (6, 2, 'Сидр по-вкуснее'),
       (7, 2, 'Сидр еще вкуснее'),
       (8, 2, 'Сидр вкуснейший');


INSERT IGNORE INTO `promo_group` (`id`, `name`)
VALUES (2, 'Сидрильная группа');
