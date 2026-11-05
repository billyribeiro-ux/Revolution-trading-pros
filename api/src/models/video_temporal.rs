//! Checked local adapters for PostgreSQL's wider date/timestamp range.
//! SQL NULL stays None; unsupported stored values are decoding errors.
use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use sqlx::{
    Decode, Postgres, Type, ValueRef,
    error::BoxDynError,
    postgres::{PgTypeInfo, PgValueFormat, PgValueRef},
};

pub struct PgSafeOptionalDate(Option<NaiveDate>);
pub struct PgSafeOptionalTimestamp(Option<DateTime<Utc>>);

fn date_from_days(days: i32) -> Result<NaiveDate, BoxDynError> {
    let epoch = NaiveDate::from_ymd_opt(2000, 1, 1).ok_or("Invalid PostgreSQL date epoch")?;
    let delta = TimeDelta::try_days(i64::from(days)).ok_or("Stored video date is out of range")?;
    epoch
        .checked_add_signed(delta)
        .ok_or_else(|| "Stored video date is outside the supported calendar range".into())
}
fn timestamp_from_microseconds(microseconds: i64) -> Result<DateTime<Utc>, BoxDynError> {
    let epoch = DateTime::<Utc>::from_timestamp(946_684_800, 0)
        .ok_or("Invalid PostgreSQL timestamp epoch")?;
    epoch
        .checked_add_signed(TimeDelta::microseconds(microseconds))
        .ok_or_else(|| "Stored video timestamp is outside the supported calendar range".into())
}

impl Type<Postgres> for PgSafeOptionalDate {
    fn type_info() -> PgTypeInfo {
        <NaiveDate as Type<Postgres>>::type_info()
    }
}
impl Type<Postgres> for PgSafeOptionalTimestamp {
    fn type_info() -> PgTypeInfo {
        <DateTime<Utc> as Type<Postgres>>::type_info()
    }
}
impl<'r> Decode<'r, Postgres> for PgSafeOptionalDate {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        if value.is_null() {
            return Ok(Self(None));
        }
        let date = match value.format() {
            PgValueFormat::Binary => {
                date_from_days(i32::from_be_bytes(value.as_bytes()?.try_into()?))?
            }
            PgValueFormat::Text => NaiveDate::parse_from_str(value.as_str()?, "%Y-%m-%d")?,
        };
        Ok(Self(Some(date)))
    }
}
impl<'r> Decode<'r, Postgres> for PgSafeOptionalTimestamp {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        if value.is_null() {
            return Ok(Self(None));
        }
        let timestamp = match value.format() {
            PgValueFormat::Binary => {
                timestamp_from_microseconds(i64::from_be_bytes(value.as_bytes()?.try_into()?))?
            }
            PgValueFormat::Text => {
                DateTime::parse_from_str(value.as_str()?, "%Y-%m-%d %H:%M:%S%.f%#z")?
                    .with_timezone(&Utc)
            }
        };
        Ok(Self(Some(timestamp)))
    }
}
impl From<PgSafeOptionalDate> for Option<NaiveDate> {
    fn from(value: PgSafeOptionalDate) -> Self {
        value.0
    }
}
impl From<PgSafeOptionalTimestamp> for Option<DateTime<Utc>> {
    fn from(value: PgSafeOptionalTimestamp) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finite_supported_calendar_values_preserve_their_exact_value() {
        let epoch = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        for date in [NaiveDate::MIN, epoch, NaiveDate::MAX] {
            assert_eq!(
                date_from_days((date - epoch).num_days().try_into().unwrap()).unwrap(),
                date
            );
        }
        let epoch = DateTime::<Utc>::from_timestamp(946_684_800, 0).unwrap();
        for micros in [-946_684_800_000_000, -1, 0, 1, 831_126_530_123_456] {
            assert_eq!(
                timestamp_from_microseconds(micros).unwrap() - epoch,
                TimeDelta::microseconds(micros)
            );
        }
    }
    #[test]
    fn postgres_infinities_and_out_of_calendar_ranges_return_errors() {
        for days in [i32::MIN, i32::MAX, 108_842_886, -108_842_886] {
            assert!(
                date_from_days(days).is_err(),
                "Unsupported DATE days: {days}"
            );
        }
        for micros in [i64::MIN, i64::MIN + 1, i64::MAX - 1, i64::MAX] {
            assert!(
                timestamp_from_microseconds(micros).is_err(),
                "Unsupported TIMESTAMPTZ microseconds: {micros}"
            );
        }
    }
    #[test]
    fn nullable_adapters_do_not_fabricate_replacement_dates() {
        assert_eq!(Option::<NaiveDate>::from(PgSafeOptionalDate(None)), None);
        assert_eq!(
            Option::<DateTime<Utc>>::from(PgSafeOptionalTimestamp(None)),
            None
        );
    }

    #[tokio::test]
    #[ignore = "Requires an explicitly enabled owned local PostgreSQL fixture"]
    async fn owned_postgres_binary_and_text_decoders_return_errors_without_panicking() {
        use sqlx::FromRow;
        assert_eq!(
            std::env::var("ALLOW_OWNED_VIDEO_TEMPORAL_PROOF").as_deref(),
            Ok("1")
        );
        let url =
            std::env::var("VIDEO_TEMPORAL_TEST_DATABASE_URL").expect("Owned fixture URL required");
        let parsed = reqwest::Url::parse(&url).unwrap();
        assert!(matches!(parsed.scheme(), "postgres" | "postgresql"));
        assert_eq!(parsed.host_str(), Some("127.0.0.1"));
        assert_eq!(parsed.port(), Some(55439));
        assert_eq!(parsed.username(), "admin_verify_fixture");
        assert!(parsed.path().starts_with("/admin_move_verify_"));
        let pool = sqlx::PgPool::connect(&url).await.unwrap();
        #[derive(FromRow)]
        struct Probe {
            #[sqlx(try_from = "PgSafeOptionalDate")]
            date: Option<NaiveDate>,
            #[sqlx(try_from = "PgSafeOptionalTimestamp")]
            timestamp: Option<DateTime<Utc>>,
        }
        for simple_text_protocol in [false, true] {
            let query = "SELECT DATE '2026-05-20' AS date,TIMESTAMPTZ '2026-05-20T17:45:30.123456+05:30' AS timestamp";
            let row = if simple_text_protocol {
                sqlx::raw_sql(query).fetch_one(&pool).await.unwrap()
            } else {
                sqlx::query(query).fetch_one(&pool).await.unwrap()
            };
            let probe = Probe::from_row(&row).unwrap();
            assert_eq!(probe.date, NaiveDate::from_ymd_opt(2026, 5, 20));
            assert_eq!(
                probe.timestamp.unwrap().to_rfc3339(),
                "2026-05-20T12:15:30.123456+00:00"
            );
            let query = "SELECT NULL::DATE AS date,NULL::TIMESTAMPTZ AS timestamp";
            let row = if simple_text_protocol {
                sqlx::raw_sql(query).fetch_one(&pool).await.unwrap()
            } else {
                sqlx::query(query).fetch_one(&pool).await.unwrap()
            };
            let probe = Probe::from_row(&row).unwrap();
            assert!(probe.date.is_none() && probe.timestamp.is_none());
            for query in [
                "SELECT DATE '300000-01-01' AS date,NULL::TIMESTAMPTZ AS timestamp",
                "SELECT 'infinity'::DATE AS date,NULL::TIMESTAMPTZ AS timestamp",
                "SELECT '-infinity'::DATE AS date,NULL::TIMESTAMPTZ AS timestamp",
                "SELECT NULL::DATE AS date,TIMESTAMPTZ '280000-01-01' AS timestamp",
                "SELECT NULL::DATE AS date,'infinity'::TIMESTAMPTZ AS timestamp",
                "SELECT NULL::DATE AS date,'-infinity'::TIMESTAMPTZ AS timestamp",
            ] {
                let row = if simple_text_protocol {
                    sqlx::raw_sql(query).fetch_one(&pool).await.unwrap()
                } else {
                    sqlx::query(query).fetch_one(&pool).await.unwrap()
                };
                assert!(
                    matches!(Probe::from_row(&row), Err(sqlx::Error::ColumnDecode { .. })),
                    "Unsupported stored temporal value must be a normal decode error: {query}"
                );
            }
        }
        pool.close().await;
    }
}
