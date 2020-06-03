use crate::decode::Decode;
use crate::postgres::protocol::TypeId;
use crate::postgres::type_info::PgTypeInfo;
use crate::postgres::types::raw::PgRecordDecoder;
use crate::postgres::value::PgValue;
use crate::postgres::Postgres;
use crate::types::Type;

macro_rules! impl_pg_record_for_tuple {
    ($( $idx:ident : $T:ident ),+) => {
        impl<$($T,)+> Type<Postgres> for ($($T,)+) {
            #[inline]
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::new(TypeId::RECORD, "RECORD")
            }
        }

        impl<$($T,)+> Type<Postgres> for [($($T,)+)] {
            #[inline]
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::new(TypeId::ARRAY_RECORD, "RECORD[]")
            }
        }


        impl<$($T,)+> Type<Postgres> for Vec<($($T,)+)> {
            #[inline]
            fn type_info() -> PgTypeInfo {
                <[($($T,)+)] as Type<Postgres>>::type_info()
            }
        }

        impl<'de, $($T,)+> Decode<'de, Postgres> for ($($T,)+)
        where
            $($T: 'de,)+
            $($T: Type<Postgres>,)+
            $($T: for<'tup> Decode<'tup, Postgres>,)+
        {
            fn decode(value: PgValue<'de>) -> crate::Result<Self> {
                let mut decoder = PgRecordDecoder::new(value)?;

                $(let $idx: $T = decoder.decode()?;)+

                Ok(($($idx,)+))
            }
        }
    };
}

impl_pg_record_for_tuple!(_1: T1);

impl_pg_record_for_tuple!(_1: T1, _2: T2);

impl_pg_record_for_tuple!(_1: T1, _2: T2, _3: T3);

impl_pg_record_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4);

impl_pg_record_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4, _5: T5);

impl_pg_record_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4, _5: T5, _6: T6);

impl_pg_record_for_tuple!(_1: T1, _2: T2, _3: T3, _4: T4, _5: T5, _6: T6, _7: T7);

impl_pg_record_for_tuple!(
    _1: T1,
    _2: T2,
    _3: T3,
    _4: T4,
    _5: T5,
    _6: T6,
    _7: T7,
    _8: T8
);

impl_pg_record_for_tuple!(
    _1: T1,
    _2: T2,
    _3: T3,
    _4: T4,
    _5: T5,
    _6: T6,
    _7: T7,
    _8: T8,
    _9: T9
);
