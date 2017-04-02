struct Csv {
    rows: Vec<Vec<String>>,
}

impl Csv {
    fn new(rows: Vec<Vec<String>>) -> Csv {
        Csv { rows: rows }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_csv() {
        let row = vec![vec!["foo".to_string(), "bar".to_string()]];
        let csv = Csv::new(row);

        assert_eq!("foo".to_string(), csv.rows[0][0]);
        panic!()
    }
}
