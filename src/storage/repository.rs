use crate::domain::log::LogRecord;
use sqlx::{Pool, Postgres, QueryBuilder};

#[derive(Clone)]
pub struct LogRepository {
    pool: Pool<Postgres>,
}

impl LogRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn insert_batch(&self, logs: Vec<LogRecord>) -> Result<(), sqlx::Error> {
        if logs.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO logs (
                timestamp,
                trace_id,
                span_id,
                trace_flags,
                severity_text,
                severity_number,
                body,
                resource_attributes,
                scope_attributes,
                log_attributes,
                service_name
            ) ",
        );

        query_builder.push_values(logs, |mut b, log| {
            // Convert nanoseconds to DateTime<Utc>
            // This is a bit rough, assuming i64 fits.
            // Start with seconds
            let seconds = log.time_unix_nano / 1_000_000_000;
            let nanos = (log.time_unix_nano % 1_000_000_000) as u32;
            let timestamp = chrono::DateTime::from_timestamp(seconds, nanos)
                .unwrap_or(chrono::DateTime::from_timestamp(0, 0).unwrap());

            b.push_bind(timestamp)
                .push_bind(log.trace_id)
                .push_bind(log.span_id)
                .push_bind(log.trace_flags)
                .push_bind(log.severity_text)
                .push_bind(log.severity_number)
                .push_bind(sqlx::types::Json(log.body))
                .push_bind(sqlx::types::Json(log.resource_attributes))
                .push_bind(sqlx::types::Json(log.scope_attributes))
                .push_bind(sqlx::types::Json(log.log_attributes))
                .push_bind(log.service_name);
        });

        let query = query_builder.build();
        query.execute(&self.pool).await?;

        Ok(())
    }
}
