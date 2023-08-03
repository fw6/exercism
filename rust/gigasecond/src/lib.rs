use time::ext::NumericalDuration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // https://docs.rs/time/latest/src/time/primitive_date_time.rs.html#822
    start + 1e9.seconds()
}
