use std::ops::Deref;

use diesel::MysqlConnection;
use rocket_contrib::json::Json;

use crate::base::{MaxCleaner, RespErrors, ValidateFormatter};
use crate::guard::{DbConn, SupplierId};
use crate::model::{BasicProduct, Product, RespProduct, Unit};

#[post(
    "/get-product-units",
    format = "application/json",
    data = "<product_names>"
)]
fn get_product_units(
    supplier_id: SupplierId,
    db_conn: DbConn,
    product_names: Json<Vec<String>>,
) -> Result<Json<CatalogUnits>, Json<RespErrors>> {
    // refactor this some day
    let mut products = {
        let mut products: Vec<BasicProduct> = vec![];
        let mut errors: Vec<(String, String)> = vec![];

        let cleaner = MaxCleaner::default();

        for name in product_names.into_inner() {
            let mut input_product = BasicProduct {
                name,
                unit: Unit::Unknown, // to pass validation
            };
            input_product.filter(&cleaner);

            if let Err(mut resp_errors) = input_product.run_validator() {
                errors.append(&mut resp_errors.errors);
            }

            products.push(input_product);
        }

        if !errors.is_empty() {
            return Err(Json(RespErrors { errors }));
        }
        products
    };

    for product in &mut products {
        product.set_unit_from_db(supplier_id.0, db_conn.deref());
    }

    Ok(Json(CatalogUnits(products)))
}

#[derive(Debug, Serialize, Deserialize)]
struct CatalogUnits(Vec<BasicProduct>);

impl CatalogUnits {
    pub fn clean(&mut self) {
        let cleaner = MaxCleaner::default();
        for basic_product in &mut self.0 {
            basic_product.filter(&cleaner);
        }
    }

    pub fn rocket_validate(&self) -> Result<(), Json<RespErrors>> {
        let mut errors: Vec<(String, String)> = vec![];

        for basic_product in &self.0 {
            if let Err(mut form_errors) = basic_product.run_validator() {
                errors.append(&mut form_errors.errors);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Json(RespErrors { errors }))
        }
    }

    pub fn insert_or_update(self, supplier_id: u32, conn: &MysqlConnection) {
        for basic_product in self.0 {
            basic_product
                .into_new_product(supplier_id)
                .insert_or_update(conn);
        }
    }
}

#[post(
    "/set-product-units",
    format = "application/json",
    data = "<catalog_units>"
)]
fn set_product_units(
    supplier_id: SupplierId,
    db_conn: DbConn,
    catalog_units: Json<CatalogUnits>,
) -> Result<String, Json<RespErrors>> {
    let mut catalog_units = catalog_units.into_inner();
    catalog_units.clean();
    catalog_units.rocket_validate()?;
    catalog_units.insert_or_update(supplier_id.into(), db_conn.deref());

    Ok(format!("{}", true))
}

#[derive(Debug, Serialize)]
struct Count {
    count: i64,
}

impl Count {
    fn new_basic_products_non_deleted(supplier_id: u32, conn: &MysqlConnection) -> Self {
        Count {
            count: Product::count_basic_non_deleted(supplier_id, conn),
        }
    }
}

#[post(
    "/replace-basic-products",
    format = "application/json",
    data = "<product_names>"
)]
fn replace_basic_products(
    supplier_id: SupplierId,
    db_conn: DbConn,
    product_names: Json<Vec<String>>,
) -> Json<Count> {
    // no need to validate each name length as mysql make only update
    Product::replace_basic_products(supplier_id.0, &product_names, db_conn.deref());
    Json(Count::new_basic_products_non_deleted(
        supplier_id.into(),
        db_conn.deref(),
    ))
}

#[post(
    "/add-basic-products",
    format = "application/json",
    data = "<product_names>"
)]
fn add_basic_products(
    supplier_id: SupplierId,
    db_conn: DbConn,
    product_names: Json<Vec<String>>,
) -> Json<Count> {
    // no need to validate each name length as mysql make only update
    Product::add_basic_products(supplier_id.0, &product_names, db_conn.deref());
    Json(Count::new_basic_products_non_deleted(
        supplier_id.into(),
        db_conn.deref(),
    ))
}

// Nowhere used for now...
#[get("/get-products")]
fn get_products(supplier_id: SupplierId, db_conn: DbConn) -> Json<Vec<RespProduct>> {
    let resp_products = RespProduct::get_supplier_products(supplier_id.into(), db_conn.deref());
    Json(resp_products)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_products,
        get_product_units,
        set_product_units,
        replace_basic_products,
        add_basic_products,
    ]
}
