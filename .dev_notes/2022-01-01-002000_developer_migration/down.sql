# !If you have edited this file, maybe you will need to copy it to '.dev_notes/2019-09-23-002000_developer_migration'
# So you cold push it into git

# delete fictitious user session
DELETE FROM `session` WHERE `session`.`hash` = 'gfasdfgdyjk78o689olymked56u24yhqefygws5r';

TRUNCATE `laxtop`.`product`