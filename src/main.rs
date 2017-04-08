use std::fs::File;
use std::io::Read;

mod csv_sorter;
use csv_sorter::Csv;


fn main() {
    let csv_contents = get_csv_records("data.csv");
    //println!("{:?}", csv_contents);

    let mut csv = Csv::new(csv_contents);
    csv.sort_by("age".to_string());
    println!("{:?}", csv.rows);
}

fn get_csv_records(filename: &str) -> Vec<Vec<String>> {
    let mut csv_contents = String::new();
    let mut csv_file = File::open(filename).expect("can't open file!");
    csv_file
        .read_to_string(&mut csv_contents)
        .expect("can't read file!");

    csv_contents
        .split('\n')
        .take_while(|&line| line.len() > 0)
        .map(|line| line.split(',').map(|col| col.to_string()).collect())
        .collect()
}
