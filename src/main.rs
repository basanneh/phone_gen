use std::io::{self, Write};

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?; // make sure prompt shows
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn is_all_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn pow10(n: u32) -> u64 {
    // safe for n up to 19 within u64 range (10^19 overflows u64),
    // but phone-number use cases are usually far smaller.
    10u64.pow(n)
}

fn main() -> io::Result<()> {
    let total_digits_str = read_line("Total digits in the phone number (e.g., 7): ")?;
    let total_digits: usize = match total_digits_str.parse() {
        Ok(v) if v > 0 => v,
        _ => {
            eprintln!("Invalid total digits. Provide a positive integer.");
            return Ok(());
        }
    };

    let prefix = read_line("Constant prefix digits (e.g., 7 or 771 or 220771): ")?;
    if !is_all_digits(&prefix) {
        eprintln!("Prefix must be digits only (no +, spaces, or dashes).");
        return Ok(());
    }

    if prefix.len() > total_digits {
        eprintln!(
            "Prefix length ({}) cannot be greater than total digits ({}).",
            prefix.len(),
            total_digits
        );
        return Ok(());
    }

    let remaining = total_digits - prefix.len();
    if remaining == 0 {
        // Nothing to generate; prefix is the full number
        println!("{prefix}");
        return Ok(());
    }

    // Guardrails: generating 10^remaining can explode fast.
    // Example: remaining=6 => 1,000,000 numbers (fine)
    // remaining=8 => 100,000,000 numbers (big)
    let total_to_generate = pow10(remaining as u32);
    eprintln!(
        "Generating {} numbers (prefix='{}', remaining_digits={}) ...",
        total_to_generate, prefix, remaining
    );

    for i in 0..total_to_generate {
        // zero-pad suffix to exactly `remaining` digits
        let suffix = format!("{:0width$}", i, width = remaining);
        println!("{}{}", prefix, suffix);
    }

    Ok(())
}
