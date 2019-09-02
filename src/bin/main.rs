use std::io::BufReader;

use rit_course_parser::{RecordReader, ClassRecord};

fn main() {
    let filename = std::env::args().nth(1).expect("should get filename");
    let mut file = std::fs::File::open(&filename).expect("should open file");
    let mut reader = RecordReader::new(BufReader::new(&mut file));

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .flexible(true)
        .from_reader(&mut reader);

    let results = csv_reader.deserialize::<ClassRecord>()
        .filter_map(Result::ok);

    for record in results {
        println!("{:?}", record);
    }
}
