use std::error::Error;
use std::{fmt, fmt::Display, io::Write, usize};

#[derive(Debug)]
pub struct Worker<'a> {
    search_term: String,
    data: &'a str,
}

impl<'a> Worker<'a> {
    pub fn new(data: &'a str, search_term: String) -> Worker {
        if search_term.is_empty() {
            panic!("Cannot search with an empty string");
        }

        Worker { data, search_term }
    }

    pub fn run<T: Write>(&self, case_sensitive: bool, out: &mut T) -> Result<usize, Box<dyn Error>> {
        write!(
            out,
            "------- Running Match Case: \"{}\" -------\n",
            self.search_term
        )?;

        let matches = match_query(case_sensitive, &self.search_term, self.data);
        for m in matches.iter() {
            write!(out, "{}\n", m)?;
        }

        Ok(matches.len())
    }
}

fn match_query_case_insensitive<'a>(query: &'a str, data: &'a str) -> Vec<Match<'a>> {
    let mut n = 0;
    let mut res = Vec::<Match<'a>>::new();
    let l_query = query.to_lowercase();

    for line in data.lines() {
        if line.to_lowercase().contains(&l_query) {
            res.push(Match {
                line_n: n,
                line: &line,
                exact: false,
                search_term: query,
            })
        }
        n += 1
    }

    res
}

fn match_query<'a>(exact: bool, query: &'a str, data: &'a str) -> Vec<Match<'a>> {
    if !exact {
        return match_query_case_insensitive(query, data);
    }

    let mut n = 0;
    let mut res = Vec::<Match>::new();

    for line in data.lines() {
        if line.contains(query) {
            res.push(Match {
                line_n: n,
                line: &line,
                exact,
                search_term: query,
            })
        }

        n += 1
    }

    res
}

struct Match<'a> {
    line_n: u32,
    line: &'a str,
    exact: bool,
    search_term: &'a str,
}

impl<'a> Display for Match<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Match: {} on line ({}): {}\texact: {}",
            self.search_term, self.line_n, self.line, self.exact
        )
    }
}





// FIRST 100 LINES!!111! 🦀🦀🦀🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳 (cc: T-Dark#9470)
