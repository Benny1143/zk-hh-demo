use risc0_zkvm::guest::env;

fn main() {
    let mut birth: [u32; 3] = [0; 3];
    env::read_slice(&mut birth);

    let mut today: [u32; 3] = [0; 3];
    env::read_slice(&mut today);

    // Split birth back to day, month, and year
    let birthday = birth[0];
    let birthmonth = birth[1];
    let birthyear = birth[2];

    // Split today data back to day, month, and year
    let day = today[0];
    let month = today[1];
    let year = today[2];

    use chrono::prelude::*;

    let birth_date = NaiveDate::from_ymd_opt(birthyear as i32, birthmonth, birthday)
        .expect("Invalid birth date");
    let today_date = NaiveDate::from_ymd_opt(year as i32, month, day).expect("Invalid today date");

    let mut age = today_date.year() - birth_date.year();
    if (today_date.month(), today_date.day()) < (birth_date.month(), birth_date.day()) {
        age -= 1;
    }

    //You are 18 years old or older.
    let flag: bool = age >= 18;

    // write public output to the journal
    use hhdemo_core::PublicJournalData;
    env::commit(&PublicJournalData {
        flag,
        day,
        month,
        year,
    });
}
