use diesel::sql_types::{Tinyint, Unsigned, Varchar};
use diesel::MysqlConnection;
use validator::{Validate, ValidationError};

use crate::base::MaxCleaner;
use crate::model::{NewProduct, Unit};

#[derive(Debug, QueryableByName, Serialize, Deserialize, Validate)]
pub struct BasicProduct {
    #[sql_type = "Varchar"]
    #[validate(
        length(min = 1, max = 255, message = "Invalid length"),
        custom = "check_lack_html"
    )]
    pub name: String,
    #[sql_type = "Unsigned<Tinyint>"]
    pub unit: Unit,
}

impl BasicProduct {
    pub fn into_new_product(self, supplier_id: u32) -> NewProduct {
        let BasicProduct { name, unit } = self;

        NewProduct {
            name,
            supplier_id,
            key: "".to_string(),
            unit,
            is_deleted: false,
        }
    }

    pub fn filter(&mut self, cleaner: &MaxCleaner) {
        if let Some(new_name) = cleaner.clean_all(&self.name) {
            self.name = new_name;
        }
    }

    pub fn set_unit_from_db(&mut self, supplier_id: u32, conn: &MysqlConnection) {
        self.unit = NewProduct::get_unit_by_name(&self.name, supplier_id, conn);
    }
}

fn check_lack_html(product_name: &str) -> Result<(), ValidationError> {
    if product_name.contains('>') {
        Err(ValidationError::new("Недопустимый символ: '>'"))
    } else if product_name.contains('<') {
        Err(ValidationError::new("Недопустимый символ: '<'"))
    } else {
        Ok(())
    }
}
