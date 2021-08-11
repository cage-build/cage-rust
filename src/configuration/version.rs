use super::Error;

#[derive(std::cmp::PartialEq, Debug)]
pub enum Version {
    V0,
}

impl Version {
    // Get the version from the file.
    pub fn get(file_content: &str) -> Result<(Version, &str, usize), Error> {
        let (h, f, line) = Version::get_line(file_content).ok_or(Error::VersionNotFound)?;
        let v = match h {
            "CAGE-BUILD-0" => Version::V0,
            _ => Err(Error::VersionUnknown(h.to_string()))?,
        };

        Ok((v, f, line))
    }

    /// Get the header, the rest of the file and the line number.
    fn get_line(file_content: &str) -> Option<(&str, &str, usize)> {
        let mut line = 1;
        let mut comment: bool = false;
        for (i, c) in file_content.char_indices() {
            if c == '\n' {
                comment = false;
                line += 1;
            } else if c == '#' {
                comment = true;
            } else if comment || c.is_whitespace() {
            } else {
                let f = &file_content[i..];

                let separator = match f.match_indices('\n').next() {
                    Some((separator, _)) => separator,
                    None => return None,
                };

                let h = &f[..separator];
                let h = match h.split_once('#') {
                    Some((h, _)) => h,
                    None => h,
                }
                .trim();

                return Some((h, &f[separator + 1..], line + 1));
            }
        }

        None
    }
}

#[test]
fn version_get() {
    let (v, f, l) = Version::get(
        r"# bla bla

	    # sperate comment

CAGE-BUILD-0


The rest of the
config file
...
",
    )
    .unwrap();

    assert_eq!(Version::V0, v);
    assert_eq!(6, l);
    assert_eq!(
        r"

The rest of the
config file
...
",
        f
    );

    let (v, f, l) = Version::get(
        r"CAGE-BUILD-0 # bla bla
",
    )
    .unwrap();

    assert_eq!(Version::V0, v);
    assert_eq!(2, l);
    assert_eq!("", f);
}
