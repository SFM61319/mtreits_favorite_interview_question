use mtreits_favorite_interview_question::find_common;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 4 {
        println!("Usage: cargo run --release -- <file1>.bin <file2>.bin <file_out>.bin");
        return Ok(());
    }

    let bin1 = &argv[1];
    let bin2 = &argv[2];
    let bin_out = &argv[3];

    find_common(bin1, bin2, bin_out);
    Ok(())
}
