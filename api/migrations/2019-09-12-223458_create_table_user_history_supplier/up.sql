# products that was at least one time ordered by user
# these products need to be saved for user

CREATE TABLE `user_history_supplier`
(
    `user_id`     INT UNSIGNED NOT NULL,
    `supplier_id` INT UNSIGNED NOT NULL,
    PRIMARY KEY (`user_id`, `supplier_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;


/*
SELECT product.id, product.supplier_id, product.name
FROM product
WHERE product.supplier_id IN (1, 2, 3)
  AND (
        product.id IN (1, 2, 3)
        OR (product.is_deleted = false)
    )
*/
