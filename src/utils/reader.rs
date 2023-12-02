use std::{fs::File, io::BufReader};

pub fn read_file_line_by_line(
    filepath: &str,
) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    Ok(reader)
}
