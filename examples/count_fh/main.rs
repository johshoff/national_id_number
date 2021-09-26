use national_id_number::calculate_checksum;

/// count the number of valid FH-numbers
fn main() {
    let fh_numbers = (800000_000..1_000000_000)
        .filter(|n| calculate_checksum(*n).is_some())
        .count();

    println!("There are {} valid FH-numbers", fh_numbers);
}
