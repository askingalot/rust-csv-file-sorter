
pub struct Csv {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Csv {
    pub fn new(mut rows: Vec<Vec<String>>) -> Csv {
        let header = rows.remove(0);
        Csv { header: header, rows: rows }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn build_csv() -> Csv {
        let header = vec!["Col1", "Col2"];
        let row1 = vec!["foo", "bar"];
        let row2 = vec!["baz", "qux"];
        let csv_data = vec![header, row1, row2];

        let string_csv_data =
            csv_data
                .iter()
                .map(|row| row.iter().map(|col| col.to_string()).collect())
                .collect();

        Csv::new( string_csv_data )
    }

    #[test]
    fn new_csv() {
        let csv = build_csv();

        assert_eq!("foo".to_string(), csv.rows[0][0]);
    }

    #[test]
    fn first_row_is_header() {
        let csv = build_csv();

        assert_eq!("Col1".to_string(), csv.header[0]);
    }
}
