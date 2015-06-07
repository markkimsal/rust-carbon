extern crate time;

pub use time::*;

pub struct DateTime {
    pub t: Tm,
}

impl DateTime {
    pub fn now() -> DateTime {
        DateTime {
            t: time::now(),
        }
    }

    pub fn create_from_tm(tm: Tm) -> DateTime {
        DateTime {
            t: tm,
        }
    }

    pub fn to_string(&self) -> time::TmFmt {
        self.t.ctime()
    }

    pub fn unixtime(&self) -> time::Timespec {
        self.t.to_timespec()
    }

    pub fn start_of_month(&self) -> DateTime {
        let mut copied_tm = self.t;
        copied_tm.tm_mday = 1;
        copied_tm.tm_wday = self.t.tm_wday - (self.t.tm_mday % 7) + 1;
        // TODO: fix tm_yday
        DateTime::create_from_tm(copied_tm).start_of_day()
    }

    pub fn start_of_day(&self) -> DateTime {
        let mut copied_tm = self.t;
        copied_tm.tm_hour = 0;
        copied_tm.tm_sec = 0;
        copied_tm.tm_min = 0;
        copied_tm.tm_nsec = 0;
        // TODO: fix tm_isdst

        DateTime::create_from_tm(copied_tm)
    }

    // create Tm from unixtime
//    pub fn at(clock: Timespec) -> DateTime {
//    }
}
