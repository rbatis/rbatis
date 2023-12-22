use crate::column::PgColumn;
use crate::connection::PgConnection;
use crate::message::{ParameterDescription, RowDescription};
use crate::type_info::{PgCustomType, PgType, PgTypeInfo, PgTypeKind};
use crate::types::Oid;
use futures_core::future::BoxFuture;
use rbdc::db::Connection;
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use rbs::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Describes the type of the `pg_type.typtype` column
///
/// See <https://www.postgresql.org/docs/13/catalog-pg-type.html>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TypType {
    Base,
    Composite,
    Domain,
    Enum,
    Pseudo,
    Range,
}

impl TryFrom<u8> for TypType {
    type Error = ();

    fn try_from(t: u8) -> Result<Self, Self::Error> {
        let t = match t {
            b'b' => Self::Base,
            b'c' => Self::Composite,
            b'd' => Self::Domain,
            b'e' => Self::Enum,
            b'p' => Self::Pseudo,
            b'r' => Self::Range,
            _ => return Err(()),
        };
        Ok(t)
    }
}

/// Describes the type of the `pg_type.typcategory` column
///
/// See <https://www.postgresql.org/docs/13/catalog-pg-type.html#CATALOG-TYPCATEGORY-TABLE>
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TypCategory {
    Array,
    Boolean,
    Composite,
    DateTime,
    Enum,
    Geometric,
    Network,
    Numeric,
    Pseudo,
    Range,
    String,
    Timespan,
    User,
    BitString,
    Unknown,
}

impl TryFrom<u8> for TypCategory {
    type Error = ();

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        let c = match c {
            b'A' => Self::Array,
            b'B' => Self::Boolean,
            b'C' => Self::Composite,
            b'D' => Self::DateTime,
            b'E' => Self::Enum,
            b'G' => Self::Geometric,
            b'I' => Self::Network,
            b'N' => Self::Numeric,
            b'P' => Self::Pseudo,
            b'R' => Self::Range,
            b'S' => Self::String,
            b'T' => Self::Timespan,
            b'U' => Self::User,
            b'V' => Self::BitString,
            b'X' => Self::Unknown,
            _ => return Err(()),
        };
        Ok(c)
    }
}

impl PgConnection {
    pub(super) async fn handle_row_description(
        &mut self,
        desc: Option<RowDescription>,
        should_fetch: bool,
    ) -> Result<(Vec<PgColumn>, HashMap<UStr, usize>), Error> {
        let mut columns = Vec::with_capacity(100);
        let mut column_names = HashMap::with_capacity(100);

        let desc = if let Some(desc) = desc {
            desc
        } else {
            // no rows
            return Ok((columns, column_names));
        };

        columns.reserve(desc.fields.len());
        column_names.reserve(desc.fields.len());

        for (index, field) in desc.fields.into_iter().enumerate() {
            let name = UStr::from(field.name);

            let type_info = self
                .maybe_fetch_type_info_by_oid(field.data_type_id, should_fetch)
                .await?;

            let column = PgColumn {
                ordinal: index,
                name: name.clone(),
                type_info,
                relation_id: field.relation_id,
                relation_attribute_no: field.relation_attribute_no,
            };

            columns.push(column);
            column_names.insert(name, index);
        }

        Ok((columns, column_names))
    }

    pub(super) async fn handle_parameter_description(
        &mut self,
        desc: ParameterDescription,
    ) -> Result<Vec<PgTypeInfo>, Error> {
        let mut params = Vec::with_capacity(desc.types.len());

        for ty in desc.types {
            params.push(self.maybe_fetch_type_info_by_oid(ty, true).await?);
        }

        Ok(params)
    }

    async fn maybe_fetch_type_info_by_oid(
        &mut self,
        oid: Oid,
        should_fetch: bool,
    ) -> Result<PgTypeInfo, Error> {
        // first we check if this is a built-in type
        // in the average application, the vast majority of checks should flow through this
        if let Some(info) = PgTypeInfo::try_from_oid(oid) {
            return Ok(info);
        }

        // next we check a local cache for user-defined type names <-> object id
        if let Some(info) = self.cache_type_info.get(&oid) {
            return Ok(info.clone());
        }

        // fallback to asking the database directly for a type name
        if should_fetch {
            let info = self.fetch_type_by_oid(oid).await?;

            // cache the type name <-> oid relationship in a paired hashmap
            // so we don't come down this road again
            self.cache_type_info.insert(oid, info.clone());
            self.cache_type_oid
                .insert(info.0.name().to_string().into(), oid);

            Ok(info)
        } else {
            // we are not in a place that *can* run a query
            // this generally means we are in the middle of another query
            // this _should_ only happen for complex types sent through the TEXT protocol
            // we're open to ideas to correct this.. but it'd probably be more efficient to figure
            // out a way to "prime" the type cache for connections rather than make this
            // fallback work correctly for complex user-defined types for the TEXT protocol
            Ok(PgTypeInfo(PgType::DeclareWithOid(oid)))
        }
    }

    fn fetch_type_by_oid(&mut self, oid: Oid) -> BoxFuture<'_, Result<PgTypeInfo, Error>> {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct PGType {
            pub typname: String,
            pub typtype: i8,
            pub typcategory: i8,
            pub typrelid: u32,
            pub typelem: u32,
            pub typbasetype: i32,
        }

        Box::pin(async move {
            // let (name, typ_type, category, relation_id, element, base_type): (String, i8, i8, Oid, Oid, Oid) = query_as(
            //     "SELECT typname, typtype, typcategory, typrelid, typelem, typbasetype FROM pg_catalog.pg_type WHERE oid = $1",
            // )
            //     .bind(oid)
            //     .fetch_one(&mut *self)
            //     .await?;
            let mut pg_type = PGType {
                typname: "".to_string(),
                typtype: 0,
                typcategory: 0,
                typrelid: 0,
                typelem: 0,
                typbasetype: 0,
            };
            let rows =self.get_values("SELECT typname, typtype, typcategory, typrelid, typelem, typbasetype FROM pg_catalog.pg_type WHERE oid = $1", vec![oid.0.into()])
                .await?;
            let vs: Vec<PGType> =
                rbs::from_value(Value::Array(rows)).map_err(|e| Error::from(e.to_string()))?;
            if let Some(x) = vs.into_iter().next() {
                pg_type = x;
            }
            let typ_type = TypType::try_from(pg_type.typtype as u8);
            let category = TypCategory::try_from(pg_type.typcategory as u8);
            let typelem = Oid::from(pg_type.typelem);
            let relation_id = Oid::from(pg_type.typrelid);
            let typbasetype = Oid::from(pg_type.typbasetype as u32);
            match (typ_type, category) {
                (Ok(TypType::Domain), _) => {
                    self.fetch_domain_by_oid(oid, typbasetype, pg_type.typname)
                        .await
                }

                (Ok(TypType::Base), Ok(TypCategory::Array)) => {
                    Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                        kind: PgTypeKind::Array(
                            self.maybe_fetch_type_info_by_oid(typelem, true).await?,
                        ),
                        name: pg_type.typname.into(),
                        oid,
                    }))))
                }

                (Ok(TypType::Pseudo), Ok(TypCategory::Pseudo)) => {
                    Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                        kind: PgTypeKind::Pseudo,
                        name: pg_type.typname.into(),
                        oid,
                    }))))
                }

                (Ok(TypType::Range), Ok(TypCategory::Range)) => {
                    self.fetch_range_by_oid(oid, pg_type.typname).await
                }

                (Ok(TypType::Enum), Ok(TypCategory::Enum)) => {
                    self.fetch_enum_by_oid(oid, pg_type.typname).await
                }

                (Ok(TypType::Composite), Ok(TypCategory::Composite)) => {
                    self.fetch_composite_by_oid(oid, relation_id, pg_type.typname)
                        .await
                }

                _ => Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                    kind: PgTypeKind::Simple,
                    name: pg_type.typname.into(),
                    oid,
                })))),
            }
        })
    }

    pub(crate) async fn fetch_type_id_by_name(&mut self, name: &str) -> Result<Oid, Error> {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct V {
            pub oid: Oid,
        }
        if let Some(oid) = self.cache_type_oid.get(name) {
            return Ok(*oid);
        }
        //language=SQL
        let mut oid = Oid(0);
        let rows = self
            .get_values(
                "SELECT oid FROM pg_catalog.pg_type WHERE typname ILIKE $1",
                vec![Value::String(name.to_string())],
            )
            .await
            .map_err(|_| Error::from("TypeNotFound:".to_string() + name))?;
        let vs: Vec<V> =
            rbs::from_value(Value::Array(rows)).map_err(|e| Error::from(e.to_string()))?;
        if let Some(x) = vs.into_iter().next() {
            oid = x.oid;
        }
        self.cache_type_oid.insert(name.to_string().into(), oid);
        Ok(oid)
    }

    fn fetch_domain_by_oid(
        &mut self,
        oid: Oid,
        base_type: Oid,
        name: String,
    ) -> BoxFuture<'_, Result<PgTypeInfo, Error>> {
        Box::pin(async move {
            let base_type = self.maybe_fetch_type_info_by_oid(base_type, true).await?;

            Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                oid,
                name: name.into(),
                kind: PgTypeKind::Domain(base_type),
            }))))
        })
    }

    fn fetch_range_by_oid(
        &mut self,
        oid: Oid,
        name: String,
    ) -> BoxFuture<'_, Result<PgTypeInfo, Error>> {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct V {
            pub rngsubtype: Oid,
        }
        Box::pin(async move {
            let rows = self
                .get_values(
                    r#"
SELECT rngsubtype
FROM pg_catalog.pg_range
WHERE rngtypid = $1
                "#,
                    vec![oid.0.into()],
                )
                .await?;
            let vs: Vec<V> =
                rbs::from_value(Value::Array(rows)).map_err(|e| Error::from(e.to_string()))?;
            let mut element_oid = Oid(0);
            if let Some(x) = vs.into_iter().next() {
                element_oid = x.rngsubtype;
            }
            let element = self.maybe_fetch_type_info_by_oid(element_oid, true).await?;

            Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                kind: PgTypeKind::Range(element),
                name: name.into(),
                oid,
            }))))
        })
    }

    async fn fetch_enum_by_oid(&mut self, oid: Oid, name: String) -> Result<PgTypeInfo, Error> {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct V {
            pub enumlabel: String,
        }
        let rows = self
            .get_values(
                r#"
SELECT enumlabel
FROM pg_catalog.pg_enum
WHERE enumtypid = $1
ORDER BY enumsortorder
            "#,
                vec![oid.0.into()],
            )
            .await?;
        let vs: Vec<V> =
            rbs::from_value(Value::Array(rows)).map_err(|e| Error::from(e.to_string()))?;

        let mut variants = Vec::with_capacity(vs.len());
        for x in vs {
            variants.push(x.enumlabel);
        }

        Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            oid,
            name: name.into(),
            kind: PgTypeKind::Enum(Arc::from(variants)),
        }))))
    }

    fn fetch_composite_by_oid(
        &mut self,
        oid: Oid,
        relation_id: Oid,
        name: String,
    ) -> BoxFuture<'_, Result<PgTypeInfo, Error>> {
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct V {
            pub attname: String,
            pub atttypid: Oid,
        }

        Box::pin(async move {
            let rows = self
                .get_values(
                    r#"
SELECT attname, atttypid
FROM pg_catalog.pg_attribute
WHERE attrelid = $1
AND NOT attisdropped
AND attnum > 0
ORDER BY attnum
                "#,
                    vec![relation_id.0.into()],
                )
                .await?;

            let raw_fields: Vec<V> =
                rbs::from_value(Value::Array(rows)).map_err(|e| Error::from(e.to_string()))?;

            let mut fields = Vec::new();

            for v in raw_fields.into_iter() {
                let field_type = self.maybe_fetch_type_info_by_oid(v.atttypid, true).await?;
                fields.push((v.attname, field_type));
            }

            Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
                oid,
                name: name.into(),
                kind: PgTypeKind::Composite(Arc::from(fields)),
            }))))
        })
    }
}
