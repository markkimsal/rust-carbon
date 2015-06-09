extern crate carbon;
extern crate time;

#[test]
fn test_start_of_month() {
    let january_ends = time::Tm {
        tm_sec: 59,
        tm_min: 59,
        tm_hour: 23,
        tm_mday: 2,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 5,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 1,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 4,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    // TODO: add test case when tm_wday is 0
    assert_eq!(january_starts, carbon::DateTime::create_from_tm(january_ends).start_of_month().t);
}

#[test]
fn test_start_of_day() {
    let january_ends = time::Tm {
        tm_sec: 59,
        tm_min: 59,
        tm_hour: 23,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    assert_eq!(january_starts, carbon::DateTime::create_from_tm(january_ends).start_of_day().t);
}

#[test]
fn test_start_of_hour() {
    let january_ends = time::Tm {
        tm_sec: 59,
        tm_min: 59,
        tm_hour: 23,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 23,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    assert_eq!(january_starts, carbon::DateTime::create_from_tm(january_ends).start_of_hour().t);
}

#[test]
fn test_start_of_minute() {
    let january_ends = time::Tm {
        tm_sec: 59,
        tm_min: 59,
        tm_hour: 23,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0,
        tm_min: 59,
        tm_hour: 23,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    assert_eq!(january_starts, carbon::DateTime::create_from_tm(january_ends).start_of_minute().t);
}

#[test]
fn test_start_of_second() {
    let january_ends = time::Tm {
        tm_sec: 59,
        tm_min: 59,
        tm_hour: 23,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 59,
        tm_min: 59,
        tm_hour: 23,
        tm_mday: 31,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 6,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    assert_eq!(january_starts, carbon::DateTime::create_from_tm(january_ends).start_of_second().t);
}
