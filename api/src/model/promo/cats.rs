use std::collections::HashMap;

use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::schema::promo_cat;

#[derive(Debug, Serialize, Deserialize)]
pub struct PromoGroup {
    name: String,
    cats: HashMap<u32, String>,
}

impl Default for PromoGroup {
    fn default() -> Self {
        PromoGroup {
            name: "No Group".into(),
            cats: HashMap::new(),
        }
    }
}

impl PromoGroup {
    pub fn select_all(conn: &MysqlConnection) -> Vec<PromoGroup> {
        let groups = PromoGroup::select_groups(conn);
        let cats = Cat::select_all(conn);

        let mut res = HashMap::<u32, PromoGroup>::with_capacity(groups.len());

        for (id, name) in groups {
            res.insert(
                id,
                PromoGroup {
                    name,
                    cats: HashMap::default(),
                },
            );
        }

        for Cat { id, group_id, name } in cats {
            if let Some(promo_group) = res.get_mut(&group_id) {
                promo_group.cats.insert(id, name);
            } else {
                res.entry(1u32)
                    .or_insert_with(PromoGroup::default)
                    .cats
                    .insert(id, name);
            }
        }

        let mut v = Vec::with_capacity(res.len());
        for (_k, promo_group) in res {
            v.push(promo_group);
        }
        v
    }

    pub fn select_groups(conn: &MysqlConnection) -> Vec<(u32, String)> {
        use crate::schema::promo_group::{self, dsl};

        promo_group::table
            .select((dsl::id, dsl::name))
            .load(conn)
            .unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Queryable, Insertable)]
#[table_name = "promo_cat"]
pub struct Cat {
    id: u32,
    group_id: u32,
    name: String,
}

impl Cat {
    pub fn select_all(conn: &MysqlConnection) -> Vec<Cat> {
        promo_cat::table.load(conn).unwrap()
    }

    pub fn check_ids_existence(ids: &[u32], conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::promo_cat::dsl;

        promo_cat::table
            .select(dsl::id)
            .filter(dsl::id.eq_any(ids))
            .load(conn)
            .unwrap()
    }
}
