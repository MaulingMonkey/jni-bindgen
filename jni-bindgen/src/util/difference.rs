use std::io::{self, *};

pub struct Difference {
    pub line_no:    u32,
    pub original:   String,
    pub rewrite:    String,
}

impl Difference {
    /// **WARNING**: leaves self in an inconsistent state on Err.
    pub fn find(original: &mut (impl BufRead + Seek), rewrite: &mut (impl BufRead + Seek)) -> io::Result<Option<Difference>> {
        original.seek(SeekFrom::Start(0))?;
        rewrite.seek(SeekFrom::Start(0))?;

        let mut original_line = String::new();
        let mut rewrite_line = String::new();

        let mut line_no = 0;
        loop {
            line_no += 1;

            let a = read_line_no_eol(original, &mut original_line)?;
            let b = read_line_no_eol(rewrite, &mut rewrite_line)?;

            if a == 0 && b ==  0 { return Ok(None); }

            if original_line != rewrite_line {
                original.seek(SeekFrom::End(0))?;
                rewrite.seek(SeekFrom::End(0))?;
                return Ok(Some(Difference { line_no, original: original_line, rewrite: rewrite_line }));
            }
        }
    }
}

fn read_line_no_eol(reader: &mut impl BufRead, buffer: &mut String) -> io::Result<usize> {
    let size = reader.read_line(buffer)?;
    while buffer.ends_with('\r') || buffer.ends_with('\n') {
        buffer.pop();
    }
    Ok(size)
}
