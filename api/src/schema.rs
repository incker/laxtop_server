table! {
    admin (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        status -> Unsigned<Tinyint>,
    }
}

table! {
    agent (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    contact_phone (id) {
        id -> Unsigned<Integer>,
        number -> Varchar,
        country_code -> Varchar,
        name -> Varchar,
        rank -> Varchar,
        status -> Unsigned<Tinyint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    country (code) {
        code -> Varchar,
        name -> Varchar,
        lng -> Float,
        lat -> Float,
    }
}

table! {
    country_phone_code (country_code, phone_code) {
        country_code -> Varchar,
        phone_code -> Unsigned<Integer>,
    }
}

table! {
    image (id) {
        id -> Unsigned<Integer>,
        dir -> Varchar,
        dir2 -> Varchar,
        hash -> Varchar,
        hash2 -> Varchar,
    }
}

table! {
    invoice (id) {
        id -> Unsigned<Integer>,
        creation_id -> Unsigned<Integer>,
        supplier_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        spot_id -> Unsigned<Integer>,
        position_count -> Unsigned<Tinyint>,
        status -> Unsigned<Tinyint>,
        updated_at -> Timestamp,
    }
}

table! {
    invoice_product (invoice_id, product_id) {
        invoice_id -> Unsigned<Integer>,
        product_id -> Unsigned<Integer>,
        amount -> Unsigned<Integer>,
    }
}

table! {
    login (owner_id, owner_type) {
        owner_id -> Unsigned<Integer>,
        owner_type -> Unsigned<Tinyint>,
        lg -> Varchar,
        ps -> Varchar,
    }
}

table! {
    product (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        supplier_id -> Unsigned<Integer>,
        key -> Varchar,
        unit -> Unsigned<Tinyint>,
        is_deleted -> Bool,
    }
}

table! {
    promo (id) {
        id -> Unsigned<Integer>,
        supplier_id -> Unsigned<Integer>,
        cat_id -> Unsigned<Integer>,
        image_id -> Unsigned<Integer>,
        created_at -> Timestamp,
    }
}

table! {
    promo_cat (id) {
        id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    promo_cat_updated (id) {
        id -> Unsigned<Integer>,
        updated_at -> Timestamp,
    }
}

table! {
    promo_group (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    session (hash) {
        hash -> Varchar,
        owner_type -> Unsigned<Tinyint>,
        owner_id -> Unsigned<Integer>,
        expired_at -> Timestamp,
    }
}

table! {
    spot (id) {
        id -> Unsigned<Integer>,
        address -> Varchar,
        spot_type -> Varchar,
        spot_name -> Varchar,
        image_id -> Unsigned<Integer>,
        about -> Text,
        status -> Unsigned<Tinyint>,
        country_code -> Varchar,
        lng -> Float,
        lat -> Float,
        creator_type -> Unsigned<Tinyint>,
        creator_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spot_org (user_id, spot_id) {
        user_id -> Unsigned<Integer>,
        spot_id -> Unsigned<Integer>,
        org_type -> Varchar,
        org_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spot_supplier (spot_id, supplier_id) {
        spot_id -> Unsigned<Integer>,
        supplier_id -> Unsigned<Integer>,
    }
}

table! {
    supplier (id) {
        id -> Unsigned<Integer>,
        telegram_login -> Bigint,
        name -> Varchar,
        about -> Varchar,
        address -> Varchar,
        lng -> Float,
        lat -> Float,
        poly_lng_min -> Float,
        poly_lng_max -> Float,
        poly_lat_min -> Float,
        poly_lat_max -> Float,
        status -> Unsigned<Tinyint>,
        chat_id -> Bigint,
        telegram_user_id -> Bigint,
        shift -> Unsigned<Tinyint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    supplier_phone (supplier_id, sequence) {
        supplier_id -> Unsigned<Integer>,
        sequence -> Unsigned<Smallint>,
        position -> Varchar,
        number -> Varchar,
    }
}

table! {
    supplier_polygon (supplier_id, sequence) {
        supplier_id -> Unsigned<Integer>,
        sequence -> Unsigned<Smallint>,
        lng -> Float,
        lat -> Float,
    }
}

table! {
    supplier_sequence (user_id, spot_id, supplier_id) {
        user_id -> Unsigned<Integer>,
        spot_id -> Unsigned<Integer>,
        supplier_id -> Unsigned<Integer>,
        sequence -> Unsigned<Smallint>,
    }
}

table! {
    telegram_user (id) {
        id -> Bigint,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        photo_url -> Varchar,
        status -> Unsigned<Tinyint>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Unsigned<Integer>,
        number -> Varchar,
        country_code -> Varchar,
        name -> Varchar,
        license_accepted -> Timestamp,
        status -> Unsigned<Tinyint>,
        creator_type -> Unsigned<Tinyint>,
        creator_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_history_product (user_id, product_id) {
        user_id -> Unsigned<Integer>,
        product_id -> Unsigned<Integer>,
    }
}

table! {
    user_history_supplier (user_id, supplier_id) {
        user_id -> Unsigned<Integer>,
        supplier_id -> Unsigned<Integer>,
    }
}

table! {
    user_promo_cat (user_id, promo_cat_id) {
        user_id -> Unsigned<Integer>,
        promo_cat_id -> Unsigned<Integer>,
    }
}

table! {
    user_spot (user_id, spot_id) {
        user_id -> Unsigned<Integer>,
        spot_id -> Unsigned<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    admin,
    agent,
    contact_phone,
    country,
    country_phone_code,
    image,
    invoice,
    invoice_product,
    login,
    product,
    promo,
    promo_cat,
    promo_cat_updated,
    promo_group,
    session,
    spot,
    spot_org,
    spot_supplier,
    supplier,
    supplier_phone,
    supplier_polygon,
    supplier_sequence,
    telegram_user,
    user,
    user_history_product,
    user_history_supplier,
    user_promo_cat,
    user_spot,
);
