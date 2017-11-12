// Euler problem 19
//
//
pub fn run() -> u64 {
    use chrono::{NaiveDate, Duration, Datelike};

    let start = NaiveDate::from_ymd(1901, 1, 1);
    let end = NaiveDate::from_ymd(2000, 12, 31);

    // First we must get the next sunday.
    let mut cur = start + Duration::days(((7 - start.weekday().num_days_from_sunday()) % 7) as i64);

    let mut ans = 0;
    while cur <= end {
        if cur.day() == 1 {
            ans += 1;
        }
        cur = cur + Duration::days(7);
    }
    ans
}
