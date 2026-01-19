use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let timestamp = start.assume_utc().unix_timestamp() + 1000000000;
    let odt = time::OffsetDateTime::from_unix_timestamp(timestamp)
        .expect("Invalid timestamp");

    // 2. Extract date and time into a PrimitiveDateTime
    DateTime::new(odt.date(), odt.time())
}
