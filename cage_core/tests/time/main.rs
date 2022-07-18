#[test]
fn test_main_global() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let f = File::open("tests/time/time.txt").unwrap();
    let reader = BufReader::new(f);

    for l in reader.lines() {
        let l = l.unwrap();
        let (unix, rcf3339) = l.split_once('\t').unwrap();
        let unix = unix.parse::<u64>().unwrap();
        assert_eq!(
            rcf3339,
            format!("{}", cage_core::fs::Instant::unix(unix, 12_345_678))
        );
    }
}
