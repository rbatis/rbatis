use crate::arguments::PgArguments;
use crate::column::PgColumn;
use crate::connection::PgConnection;
use crate::message::{ParameterDescription, RowDescription};
use crate::statement::PgStatementMetadata;
use crate::type_info::{PgCustomType, PgType, PgTypeInfo, PgTypeKind};
use crate::types::Oid;
use futures_core::future::BoxFuture;
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use std::collections::HashMap;
use std::fmt::Write;
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
        let mut columns = Vec::new();
        let mut column_names = HashMap::new();

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
        Box::pin(async move {
            return todo!();
            // let (name, typ_type, category, relation_id, element, base_type): (String, i8, i8, Oid, Oid, Oid) = query_as(
            //     "SELECT typname, typtype, typcategory, typrelid, typelem, typbasetype FROM pg_catalog.pg_type WHERE oid = $1",
            // )
            //     .bind(oid)
            //     .fetch_one(&mut *self)
            //     .await?;
            //
            // let typ_type = TypType::try_from(typ_type as u8);
            // let category = TypCategory::try_from(category as u8);
            //
            // match (typ_type, category) {
            //     (Ok(TypType::Domain), _) => self.fetch_domain_by_oid(oid, base_type, name).await,
            //
            //     (Ok(TypType::Base), Ok(TypCategory::Array)) => {
            //         Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            //             kind: PgTypeKind::Array(
            //                 self.maybe_fetch_type_info_by_oid(element, true).await?,
            //             ),
            //             name: name.into(),
            //             oid,
            //         }))))
            //     }
            //
            //     (Ok(TypType::Pseudo), Ok(TypCategory::Pseudo)) => {
            //         Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            //             kind: PgTypeKind::Pseudo,
            //             name: name.into(),
            //             oid,
            //         }))))
            //     }
            //
            //     (Ok(TypType::Range), Ok(TypCategory::Range)) => {
            //         self.fetch_range_by_oid(oid, name).await
            //     }
            //
            //     (Ok(TypType::Enum), Ok(TypCategory::Enum)) => {
            //         self.fetch_enum_by_oid(oid, name).await
            //     }
            //
            //     (Ok(TypType::Composite), Ok(TypCategory::Composite)) => {
            //         self.fetch_composite_by_oid(oid, relation_id, name).await
            //     }
            //
            //     _ => Ok(PgTypeInfo(PgType::Custom(Arc::new(PgCustomType {
            //         kind: PgTypeKind::Simple,
            //         name: name.into(),
            //         oid,
            //     })))),
            // }
        })
    }

    pub(crate) async fn fetch_type_id_by_name(&mut self, name: &str) -> Result<Oid, Error> {
        if let Some(oid) = self.cache_type_oid.get(name) {
            return Ok(*oid);
        }

        return todo!();
        // language=SQL
        //         let (oid,): (Oid,) = query_as(
        //             "
        // SELECT oid FROM pg_catalog.pg_type WHERE typname ILIKE $1
        //                 ",
        //         )
        //         .bind(name)
        //         .fetch_optional(&mut *self)
        //         .await?
        //         .ok_or_else(|| Error::TypeNotFound {
        //             type_name: String::from(name),
        //         })?;

        //self.cache_type_oid.insert(name.to_string().into(), oid);
        //Ok(oid)
    }
}

fn visit_plan(plan: &Plan, outputs: &[String], nullables: &mut Vec<Option<bool>>) {
    if let Some(plan_outputs) = &plan.output {
        // all outputs of a Full Join must be marked nullable
        // otherwise, all outputs of the inner half of an outer join must be marked nullable
        if plan.join_type.as_deref() == Some("Full")
            || plan.parent_relation.as_deref() == Some("Inner")
        {
            for output in plan_outputs {
                if let Some(i) = outputs.iter().position(|o| o == output) {
                    // N.B. this may produce false positives but those don't cause runtime errors
                    nullables[i] = Some(true);
                }
            }
        }
    }

    if let Some(plans) = &plan.plans {
        if let Some("Left") | Some("Right") = plan.join_type.as_deref() {
            for plan in plans {
                visit_plan(plan, outputs, nullables);
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct Explain {
    #[serde(rename = "Plan")]
    plan: Plan,
}

#[derive(serde::Deserialize)]
struct Plan {
    #[serde(rename = "Join Type")]
    join_type: Option<String>,
    #[serde(rename = "Parent Relationship")]
    parent_relation: Option<String>,
    #[serde(rename = "Output")]
    output: Option<Vec<String>>,
    #[serde(rename = "Plans")]
    plans: Option<Vec<Plan>>,
}
