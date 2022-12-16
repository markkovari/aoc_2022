use std::str::FromStr;

use regex::Regex;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Valve {
    name: String,
    flow_rate: usize,
    leads_to_names: Vec<String>,
}

impl Valve {
    //FromStr was not a jam here

    // Valve WT has flow rate=0; tunnels lead to valves BD, FQ
    fn from_str(input: String) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();

        Self {
            name: parts[1].to_owned(),
            flow_rate: parts[4]
                .split(&['=', ';'])
                .nth(1)
                .expect("bruv")
                .parse()
                .expect("shoulda be numba"),
            leads_to_names: parts[9..]
                .iter()
                .map(|e| e.trim_end_matches(",").to_owned())
                .collect::<Vec<String>>(),
        }
    }
}

pub fn get_16_first() -> usize {
    4
}

pub fn get_16_second() -> i64 {
    5
}

#[cfg(test)]
mod test_16 {

    use super::*;

    #[test]
    fn test_parse_line() {
        let text = "Valve WT has flow rate=0; tunnels lead to valves BD, FQ".to_owned();
        let parsed = Valve::from_str(text);
        assert_eq!(parsed.flow_rate, 0);
        assert_eq!(parsed.name, "WT".to_owned());
        assert_eq!(parsed.leads_to_names, &["BD".to_owned(), "FQ".to_owned()]);
    }
}
