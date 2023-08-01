use crate::DateTime;

impl DateTime {
    pub fn format_as_iso8601(&self) -> Result<String, time::error::Format> {
        self.tm.format(&time::format_description::well_known::Iso8601::DEFAULT)
    }
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_imports)]
mod tests {
    use time_macros::datetime;

    use crate::DateTime;

    use super::*;

    #[test]
    fn test_start_of_month() {
        let carbon = DateTime::create_from_tm(datetime!(1997-11-12 9:55:06 -6:00));
        assert_eq!(
            carbon.format_as_iso8601().unwrap(),
            "1997-11-12T09:55:06.000000000-06:00"
        )
    }
}
