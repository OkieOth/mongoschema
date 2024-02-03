use std::env;

pub fn get_env_var_str(var_name: &str, default_value: &str) -> String {
    return match env::var(var_name) {
        Ok(s) => s,
        Err(_) => default_value.to_string(),
    };
}

pub fn get_env_var_int(var_name: &str, default_value: i32) -> i32 {
    return match env::var(var_name) {
        Ok(s) => s.parse::<i32>().unwrap(),
        Err(_) => default_value,
    };
}

pub fn size_str(size_in_bytes: u64) -> String {
    fn comma_part(v: u64, orig: u64, unit: &str) -> String {
        let rest = orig % 1024;
        if rest == 0 {
            format!("{} {}",v, unit)
        } else {
            let rest_procent = rest * 100 / 1024;
            match rest_procent {
                0 ..= 5 => return format!("{} {}", v , unit),
                5 ..= 15 => return format!("{}.1 {}", v , unit),
                15 ..= 25 => return format!("{}.2 {}", v , unit),
                25 ..= 35 => return format!("{}.3 {}", v , unit),
                35 ..= 45 => return format!("{}.4 {}", v , unit),
                45 ..= 55 => return format!("{}.5 {}", v , unit),
                55 ..= 65 => return format!("{}.6 {}", v , unit),
                65 ..= 75 => return format!("{}.7 {}", v , unit),
                75 ..= 85 => return format!("{}.8 {}", v , unit),
                85 ..= 95 => return format!("{}.9 {}", v , unit),
                _ => return format!("{} {}", v+1 , unit),
            }
        }
    }

    let kb = size_in_bytes / 1024;
    if kb < 1024 {
        comma_part(kb, size_in_bytes, "kB")
    } else {
        let mb = size_in_bytes / (1024 * 1024);
        if mb < 1024 {
            comma_part(mb, kb, "MB")
        } else {
            let gb = size_in_bytes / (1024 * 1024 * 1024);
            if gb < 1024 {
                comma_part(gb, mb, "GB")
            } else {
                let tb = size_in_bytes / (1024 * 1024 * 1024 * 1024);
                comma_part(tb, gb, "TB")
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_size_str() {

        // assert_eq!(size_str(1024), "1 kB");
        // assert_eq!(size_str(1025), "1.0 kB");
        // assert_eq!(size_str(2047), "2.0 kB");
        // assert_eq!(size_str(20047), "19.6 kB");
        // assert_eq!(size_str(2646047), "2.5 MB");
        // assert_eq!(size_str(2646047), "1 GB");
    }

    #[test]
    fn test_size_str_kilobytes() {
        assert_eq!(size_str(1724), "1.7 kB");
        assert_eq!(size_str(20480), "20 kB");
        assert_eq!(size_str(71200), "69.5 kB");
        assert_eq!(size_str(302400), "295.3 kB");
        assert_eq!(size_str(524288), "512 kB");
    }

    #[test]
    fn test_size_str_megabytes() {
        assert_eq!(size_str(1048576), "1 MB");
        assert_eq!(size_str(30971520), "29.5 MB");
        assert_eq!(size_str(83428800), "79.6 MB");
        assert_eq!(size_str(224857600), "214.4 MB");
        assert_eq!(size_str(536881912), "512 MB");
    }

    #[test]
    fn test_size_str_gigabytes() {
        assert_eq!(size_str(1873741824), "1.7 GB");
        assert_eq!(size_str(21474836480), "20 GB");
        assert_eq!(size_str(53997091200), "50.3 GB");
        assert_eq!(size_str(237374182400), "221.1 GB");
        assert_eq!(size_str(549755813888), "512 GB");
    }

    #[test]
    fn test_size_str_terabytes() {
        assert_eq!(size_str(1099511627776), "1 TB");
        assert_eq!(size_str(2899232555520), "2.6 TB");
        assert_eq!(size_str(64975581388800), "59.1 TB");
        assert_eq!(size_str(309951162777600), "281.9 TB");
        assert_eq!(size_str(562949953421312), "512 TB");
    }
}