use serde::Deserialize;
use std::{fs::File, io::BufReader};
use taxbit_export_rec::TaxBitExportRec;
use taxbitrec::TaxBitRec;

fn open_file(fname: &str) -> Result<File, Box<dyn std::error::Error>> {
    Ok(match File::open(fname) {
        Ok(f) => f,
        Err(e) => return Err(format!("Unable to open '{fname}'; {e}").into()),
    })
}

fn open_file_buf_reader(fname: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = open_file(fname)?;

    Ok(BufReader::new(file))
}

fn read_csv<T>(fname: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let reader = open_file_buf_reader(fname)?;
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut rec_a = Vec::<T>::new();

    for entry in csv_reader.deserialize() {
        let rec: T = entry?;
        rec_a.push(rec);
    }

    Ok(rec_a)
}

pub fn find_dropped_transactions(
    tbr_fname: &str,
    tber_fname: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let tbr_a = read_csv::<TaxBitRec>(tbr_fname)?;
    let tber_a = read_csv::<TaxBitExportRec>(tber_fname)?;

    let dropped = tbr_a.len() - tber_a.len();
    dbg!(tbr_a.len());
    dbg!(tber_a.len());
    dbg!(dropped);

    Ok(dropped)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3() {
        let tbr_3_a = read_csv::<TaxBitRec>("testdata/3.tbr.csv").unwrap();
        let tber_3_a = read_csv::<TaxBitExportRec>("testdata/3.tber.csv").unwrap();

        assert_eq!(tbr_3_a.len(), 3);
        assert_eq!(tber_3_a.len(), 3);
    }

    #[test]
    fn test_find_dropped_transactions() {
        let dropped =
            find_dropped_transactions("testdata/3.tbr.csv", "testdata/3.tber.csv").unwrap();

        assert_eq!(dropped, 0);
    }
}
