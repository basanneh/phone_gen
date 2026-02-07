use std::fs::File;
use std::io::{self, Write, BufWriter};

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn is_all_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn pow10(n: u32) -> u64 {
    10u64.pow(n)
}

fn main() -> io::Result<()> {
    // 1. Total digits
    let total_digits: usize = read_line("Total digits (e.g. 7): ")?
        .parse()
        .expect("Total digits must be a number");

    // 2. Constant prefix
    let prefix = read_line("Constant prefix (digits only): ")?;
    if !is_all_digits(&prefix) {
        panic!("Prefix must contain digits only");
    }

    if prefix.len() > total_digits {
        panic!("Prefix length cannot exceed total digits");
    }

    // 3. Output file
    let output_path = read_line("Output file (e.g. numbers.txt): ")?;
    let file = File::create(&output_path)?;
    let mut writer = BufWriter::new(file);

    let remaining = total_digits - prefix.len();

    if remaining == 0 {
        writeln!(writer, "{prefix}")?;
        writer.flush()?;
        println!("Done. Wrote 1 number to {output_path}");
        return Ok(());
    }

    let total = pow10(remaining as u32);
    println!(
        "Generating {} numbers → {}",
        total, output_path
    );

    // 4. Stream generation
    for i in 0..total {
        let suffix = format!("{:0width$}", i, width = remaining);
        writeln!(writer, "{}{}", prefix, suffix)?;
    }

    writer.flush()?;
    println!("Done. File saved: {}", output_path);

    Ok(())
}
