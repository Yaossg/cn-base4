/// Encode the bytes to CN_BASE4 in LE
pub fn encode_le(s: &[u8]) -> String {
    let mut ret = String::new();
    macro_rules! enc {
        ($x: expr, $str: expr) => {
            match $x {
                0 => $str.push_str("锟"),
                1 => $str.push_str("斤"),
                2 => $str.push_str("拷"),
                3 => $str.push_str("烫"),
                _ => unreachable!(),
            }
        };
    }
    for b in s {
        enc!((b >> 0) & 0b11, ret);
        enc!((b >> 2) & 0b11, ret);
        enc!((b >> 4) & 0b11, ret);
        enc!((b >> 6) & 0b11, ret);
    }
    ret
}

/// Try to decode the string of CN_BASE4 into raw u8
pub fn decode_le(s: &str) -> Result<Vec<u8>, &'static str> {
    let chars = s.chars().collect::<Vec<char>>();
    if chars.len() % 4 == 0 {
        let byte_count = chars.len() >> 2;
        let mut bytes = Vec::with_capacity(byte_count);
        let dec_single = |x| match x {
            '锟' => Ok(0),
            '斤' => Ok(1),
            '拷' => Ok(2),
            '烫' => Ok(3),
            _ => Err("The string contains non-锟斤拷烫 char"),
        };
        let mut it = chars.iter();
        for _ in 0..byte_count {
            let mut acc = 0;
            for i in 0..4 {
                acc |= dec_single(*it.next().unwrap())? << (i << 1);
            }
            bytes.push(acc);
        }
        Ok(bytes)
    } else {
        Err("The string is not full 锟斤拷烫 string")
    }
}

mod test {
    const LJYYS: &'static str = "ljyys";
    use crate::*;
    #[test]
    fn test_same() {
        assert_eq!(
            LJYYS.as_bytes(),
            decode_le(&encode_le(LJYYS.as_bytes())).unwrap()
        );
        assert_eq!(
            "114514".as_bytes(),
            decode_le(&encode_le("114514".as_bytes())).unwrap()
        );
    }

    #[test]
    fn test_encode() {
        assert_eq!(
            "锟烫拷斤拷拷拷斤斤拷烫斤斤拷烫斤烫锟烫斤",
            &encode_le(LJYYS.as_bytes())
        );
        assert_eq!(
            "斤锟烫锟斤锟烫锟锟斤烫锟斤斤烫锟斤锟烫锟锟斤烫锟",
            &encode_le("114514".as_bytes())
        );
    }
}
