use std::borrow::Cow;

use diesel::{update, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use validator::ValidationError;

use crate::base::{Cleaner, MaxCleaner};
use crate::model::Supplier;
use crate::schema::{supplier, supplier_phone};

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct SupplierInfo {
    #[validate(length(min = 1, max = 255, message = "Invalid length"))]
    pub name: String,
    #[validate(length(min = 0, max = 1000, message = "Invalid length"))]
    pub about: String,
    #[validate(length(min = 1, max = 255, message = "Invalid length"))]
    pub address: String,
    #[validate(custom = "validate_phones")]
    pub phones: Vec<Phone>,
}

fn validate_phones(phones: &[Phone]) -> Result<(), ValidationError> {
    if phones.is_empty() {
        Err(ValidationError::new("Нужно добавить хоть 1 телефон"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Queryable, QueryableByName, PartialEq, Serialize, Deserialize)]
#[table_name = "supplier_phone"]
pub struct Phone {
    pub position: String,
    pub number: String,
}

impl SupplierInfo {
    pub fn select_by_id(supplier_id: u32, conn: &MysqlConnection) -> Self {
        use crate::schema::supplier::dsl;

        let (name, about, address): (String, String, String) = dsl::supplier
            .select((dsl::name, dsl::about, dsl::address))
            .filter(dsl::id.eq(supplier_id))
            .first(conn)
            .unwrap();

        let phones = SupplierInfo::select_phones_by_id(supplier_id, conn);

        SupplierInfo {
            name,
            about,
            address,
            phones,
        }
    }

    pub fn clean(&mut self) {
        let cleaner = MaxCleaner::default();
        let phone_cleaner = Cleaner::new_phone_cleaner();

        if let Some(new_name) = cleaner.clean_all(&self.name) {
            self.name = new_name;
        }
        if let Some(new_address) = cleaner.clean_all(&self.address) {
            self.address = new_address;
        }

        for phone in &mut self.phones {
            let Phone { position, number } = phone;
            if let Some(new_position) = cleaner.clean_all(position) {
                phone.position = new_position;
            }
            if let Cow::Owned(new_number) = phone_cleaner.clean(number) {
                phone.number = new_number;
            }
        }

        if let Some(new_about) = cleaner.clean_lines(&self.about) {
            self.about = new_about;
        }
    }

    pub fn select_phones_by_id(supplier_id: u32, conn: &MysqlConnection) -> Vec<Phone> {
        use crate::schema::supplier_phone::dsl;

        let res: Vec<Phone> = dsl::supplier_phone
            .select((dsl::position, dsl::number))
            .filter(dsl::supplier_id.eq(supplier_id))
            .load(conn)
            .unwrap();

        res
    }

    pub fn update(&self, supplier_id: u32, conn: &MysqlConnection) {
        use crate::schema::supplier::dsl;
        Supplier::increment_supplier_shift(supplier_id, conn);

        update(supplier::table.filter(dsl::id.eq(supplier_id)))
            .set((
                dsl::name.eq(&self.name),
                dsl::about.eq(&self.about),
                dsl::address.eq(&self.address),
            ))
            .execute(conn)
            .unwrap();

        self.update_phones(supplier_id, conn);
    }

    fn update_phones(&self, supplier_id: u32, conn: &MysqlConnection) {
        use crate::schema::supplier_phone::dsl;

        self.delete_phones(supplier_id, conn);

        let values = {
            let mut sequence: u16 = 1;
            let mut values = Vec::with_capacity(self.phones.len() as usize);
            for Phone { position, number } in &self.phones {
                values.push((
                    dsl::supplier_id.eq(supplier_id),
                    dsl::sequence.eq(sequence),
                    dsl::position.eq(position),
                    dsl::number.eq(number),
                ));
                sequence += 1;
            }
            values
        };

        if !values.is_empty() {
            let _res = diesel::insert_or_ignore_into(supplier_phone::table)
                .values(&values)
                .execute(conn)
                .expect("could not insert supplier_phone");
        }
    }

    pub fn delete_phones(&self, supplier_id: u32, conn: &MysqlConnection) {
        use crate::schema::supplier_phone::dsl;
        let target = dsl::supplier_id.eq(supplier_id);
        let _res = diesel::delete(dsl::supplier_phone.filter(target))
            .execute(conn)
            .expect("Some fail here: SupplierInfo::delete_phones");
    }
}
