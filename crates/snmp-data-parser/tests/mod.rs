pub mod property {
    use snmp_data_parser::PropertyParser;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn snmpdata() {
        let mut valids = BufReader::new(File::open("./tests/resources/property-os-linux-std.txt").unwrap()).lines();

        let input = BufReader::new(File::open("./tests/resources/os-linux-std.txt").unwrap());
        let reader = PropertyParser::from_reader(input);

        for res in reader.flatten() {
            assert_eq!(format!("{:?}", res), valids.next().unwrap().unwrap());
        }
        assert!(valids.next().is_none());
    }
}

pub mod line {
    use snmp_data_parser::LineReader;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn snmpdata() {
        let mut valids = BufReader::new(File::open("./tests/resources/line-os-linux-std.txt").unwrap()).lines();

        let input = BufReader::new(File::open("./tests/resources/os-linux-std.txt").unwrap());
        let reader = LineReader::new(input);

        for line in reader {
            let output = format!("{:?}", line);
            assert_eq!(output, valids.next().unwrap().unwrap());
        }
        assert!(valids.next().is_none());
    }
}

pub mod parser {

    use snmp_data_parser::parser::snmp_data::VeraxModifierExtractor;
    use snmp_data_parser::SnmpDataParser;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn snmpdata() {
        let mut valids = BufReader::new(File::open("./tests/resources/parser-os-linux-std.txt").unwrap()).lines();

        let input = BufReader::new(File::open("./tests/resources/os-linux-std.txt").unwrap());
        let reader = SnmpDataParser::new(input, VeraxModifierExtractor {});

        for data in reader.flatten() {
            assert_eq!(format!("{:?}", data), valids.next().unwrap().unwrap());
        }
        assert!(valids.next().is_none());
    }
}
