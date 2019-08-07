use std::io::{self, Write};

pub fn write_preamble(out: &mut impl Write) -> io::Result<()> {
    write!(out, "{}", include_str!("preamble-contents.rs"))?;
    writeln!(out, "")?;
    writeln!(out, "")?;
    Ok(())
}
