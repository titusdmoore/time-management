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
    pub fn from(month: &str) -> Months {
        match month {
            "January" => Months::January,
            "February" => Months::February,
            "March" => Months::March,
            "April" => Months::April,
            "May" => Months::May,
            "June" => Months::June,
            "July" => Months::July,
            "August" => Months::August,
            "September" => Months::September,
            "October" => Months::October,
            "November" => Months::November,
            "December" => Months::December,
            _ => Months::January,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Months::January => 1,
            Months::February => 2,
            Months::March => 3,
            Months::April => 4,
            Months::May => 5,
            Months::June => 6,
            Months::July => 7,
            Months::August => 8,
            Months::September => 9,
            Months::October => 10,
            Months::November => 11,
            Months::December => 12,
        }
    }

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
