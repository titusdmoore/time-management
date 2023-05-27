use std::fmt;

#[derive(Debug)]
pub enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl fmt::Display for Months {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Months {
    pub fn get_month(month: usize) -> Months {
        match month {
            1 => Months::January,
            2 => Months::February,
            3 => Months::March,
            4 => Months::April,
            5 => Months::May,
            6 => Months::June,
            7 => Months::July,
            8 => Months::August,
            9 => Months::September,
            10 => Months::October,
            11 => Months::November,
            12 => Months::December,
            _ => Months::January,
        }
    }
}
