use csv;
use std::error::Error;
use std::path::Path;

pub fn ints_from_csv(path: String) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut results: Vec<i32> = vec![];
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(Path::new(&path))?;
    let records = reader.records();
    for record in records {
        for row in record.iter() {
            for field in row.iter() {
                results.push(field.parse::<i32>().unwrap());
            }
        }
    }
   return Ok(results);
}


#[cfg(test)]
mod tests {
    #[test]
    fn ints_from_csv_works() {
        let mut ints = crate::ints_from_csv("./test_support/test_int_csv.csv".to_string()).unwrap();
        assert_eq!(ints, vec![0, 1, 2, 3]);
        ints = crate::ints_from_csv("./test_support/test_int_csv2.csv".to_string()).unwrap();
        assert_eq!(ints, vec![0, 1, 2, 3]);
    }
}
