use crate::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;
use crate::strings::ciphertext::{ClearString, FheString, GenericPattern, UIntArg};
use crate::strings::test_functions::{result_message_clear_pat, result_message_pat};
use crate::strings::Keys;
use std::time::Instant;

const TEST_CASES_FIND: [&str; 8] = ["", "a", "abc", "b", "ab", "dabc", "abce", "dabce"];

const PATTERN_FIND: [&str; 5] = ["", "a", "b", "ab", "abc"];

#[test]
fn test_find() {
    let keys = Keys::new(PARAM_MESSAGE_2_CARRY_2);

    for str_pad in 0..2 {
        for pat_pad in 0..2 {
            for str in TEST_CASES_FIND {
                for pat in PATTERN_FIND {
                    keys.assert_find(str, Some(str_pad), pat, Some(pat_pad));
                    keys.assert_rfind(str, Some(str_pad), pat, Some(pat_pad));
                }
            }
        }
    }
}

#[test]
fn test_replace() {
    let keys = Keys::new(PARAM_MESSAGE_2_CARRY_2);

    for str_pad in 0..2 {
        for from_pad in 0..2 {
            for to_pad in 0..2 {
                for str in TEST_CASES_FIND {
                    for from in PATTERN_FIND {
                        for to in ["", " ", "a", "abc"] {
                            keys.assert_replace(
                                str,
                                Some(str_pad),
                                from,
                                Some(from_pad),
                                to,
                                Some(to_pad),
                            );
                            for n in 0..=2 {
                                for max in n..n + 2 {
                                    keys.assert_replacen(
                                        (str, Some(str_pad)),
                                        (from, Some(from_pad)),
                                        (to, Some(to_pad)),
                                        n,
                                        max,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Keys {
    pub fn assert_find(&self, str: &str, str_pad: Option<u32>, pat: &str, pat_pad: Option<u32>) {
        let expected = str.find(pat);

        let enc_str = FheString::new(&self.ck, str, str_pad);
        let enc_pat = GenericPattern::Enc(FheString::new(&self.ck, pat, pat_pad));
        let clear_pat = GenericPattern::Clear(ClearString::new(pat.to_string()));

        let start = Instant::now();
        let (index, is_some) = self.sk.find(&enc_str, &enc_pat);
        let end = Instant::now();

        let dec_index = self.ck.decrypt_radix::<u32>(&index);
        let dec_is_some = self.ck.decrypt_bool(&is_some);

        let dec = dec_is_some.then_some(dec_index as usize);

        println!("\n\x1b[1mFind:\x1b[0m");
        result_message_pat(str, pat, expected, dec, end.duration_since(start));

        assert_eq!(dec, expected);

        let start = Instant::now();
        let (index, is_some) = self.sk.find(&enc_str, &clear_pat);
        let end = Instant::now();

        let dec_index = self.ck.decrypt_radix::<u32>(&index);
        let dec_is_some = self.ck.decrypt_bool(&is_some);

        let dec = dec_is_some.then_some(dec_index as usize);

        println!("\n\x1b[1mFind:\x1b[0m");
        result_message_clear_pat(str, pat, expected, dec, end.duration_since(start));

        assert_eq!(dec, expected);
    }

    pub fn assert_rfind(&self, str: &str, str_pad: Option<u32>, pat: &str, pat_pad: Option<u32>) {
        let expected = str.rfind(pat);

        let enc_str = FheString::new(&self.ck, str, str_pad);
        let enc_pat = GenericPattern::Enc(FheString::new(&self.ck, pat, pat_pad));
        let clear_pat = GenericPattern::Clear(ClearString::new(pat.to_string()));

        let start = Instant::now();
        let (index, is_some) = self.sk.rfind(&enc_str, &enc_pat);
        let end = Instant::now();

        let dec_index = self.ck.decrypt_radix::<u32>(&index);
        let dec_is_some = self.ck.decrypt_bool(&is_some);

        let dec = dec_is_some.then_some(dec_index as usize);

        println!("\n\x1b[1mRfind:\x1b[0m");
        result_message_pat(str, pat, expected, dec, end.duration_since(start));

        assert_eq!(dec, expected);

        let start = Instant::now();
        let (index, is_some) = self.sk.rfind(&enc_str, &clear_pat);
        let end = Instant::now();

        let dec_index = self.ck.decrypt_radix::<u32>(&index);
        let dec_is_some = self.ck.decrypt_bool(&is_some);

        let dec = dec_is_some.then_some(dec_index as usize);

        println!("\n\x1b[1mRfind:\x1b[0m");
        result_message_clear_pat(str, pat, expected, dec, end.duration_since(start));

        assert_eq!(dec, expected);
    }
    pub fn assert_replace(
        &self,
        str: &str,
        str_pad: Option<u32>,
        pat: &str,
        pat_pad: Option<u32>,
        to: &str,
        to_pad: Option<u32>,
    ) {
        let expected = str.replace(pat, to);

        let enc_str = FheString::new(&self.ck, str, str_pad);
        let enc_pat = GenericPattern::Enc(FheString::new(&self.ck, pat, pat_pad));
        let clear_pat = GenericPattern::Clear(ClearString::new(pat.to_string()));
        let enc_to = FheString::new(&self.ck, to, to_pad);

        let start = Instant::now();
        let result = self.sk.replace(&enc_str, &enc_pat, &enc_to);
        let end = Instant::now();

        let dec = self.ck.decrypt_ascii(&result);

        println!(
            "\n\x1b[1mReplace:\x1b[0m\n\
            \x1b[1;32m--------------------------------\x1b[0m\n\
            \x1b[1;32;1mString: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mFrom: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mTo: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mClear API Result: \x1b[0m{:?}\n\
            \x1b[1;32;1mT-fhe API Result: \x1b[0m{:?}\n\
            \x1b[1;34mExecution Time: \x1b[0m{:?}\n\
            \x1b[1;32m--------------------------------\x1b[0m",
            str,
            pat,
            to,
            expected,
            dec,
            end.duration_since(start),
        );

        assert_eq!(dec, expected);

        let start = Instant::now();
        let result = self.sk.replace(&enc_str, &clear_pat, &enc_to);
        let end = Instant::now();

        let dec = self.ck.decrypt_ascii(&result);

        println!(
            "\n\x1b[1mReplace:\x1b[0m\n\
            \x1b[1;32m--------------------------------\x1b[0m\n\
            \x1b[1;32;1mString: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mFrom (clear): \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mTo: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mClear API Result: \x1b[0m{:?}\n\
            \x1b[1;32;1mT-fhe API Result: \x1b[0m{:?}\n\
            \x1b[1;34mExecution Time: \x1b[0m{:?}\n\
            \x1b[1;32m--------------------------------\x1b[0m",
            str,
            pat,
            to,
            expected,
            dec,
            end.duration_since(start),
        );

        assert_eq!(dec, expected);
    }

    pub fn assert_replacen(
        &self,
        str: (&str, Option<u32>),
        pat: (&str, Option<u32>),
        to: (&str, Option<u32>),
        n: u16,
        max: u16,
    ) {
        let (str, str_pad) = (str.0, str.1);
        let (pat, pat_pad) = (pat.0, pat.1);
        let (to, to_pad) = (to.0, to.1);

        let expected = str.replacen(pat, to, n as usize);

        let enc_str = FheString::new(&self.ck, str, str_pad);
        let enc_pat = GenericPattern::Enc(FheString::new(&self.ck, pat, pat_pad));
        let clear_pat = GenericPattern::Clear(ClearString::new(pat.to_string()));
        let enc_to = FheString::new(&self.ck, to, to_pad);

        let clear_n = UIntArg::Clear(n);
        let enc_n = UIntArg::Enc(self.ck.encrypt_u16(n, Some(max)));

        let start = Instant::now();
        let result = self.sk.replacen(&enc_str, &enc_pat, &enc_to, &clear_n);
        let end = Instant::now();

        let dec = self.ck.decrypt_ascii(&result);

        println!(
            "\n\x1b[1mReplacen:\x1b[0m\n\
            \x1b[1;32m--------------------------------\x1b[0m\n\
            \x1b[1;32;1mString: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mFrom: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mTo: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mTimes (clear): \x1b[0m{}\n\
            \x1b[1;32;1mClear API Result: \x1b[0m{:?}\n\
            \x1b[1;32;1mT-fhe API Result: \x1b[0m{:?}\n\
            \x1b[1;34mExecution Time: \x1b[0m{:?}\n\
            \x1b[1;32m--------------------------------\x1b[0m",
            str,
            pat,
            to,
            n,
            expected,
            dec,
            end.duration_since(start),
        );
        assert_eq!(dec, expected);

        let start = Instant::now();
        let result = self.sk.replacen(&enc_str, &clear_pat, &enc_to, &enc_n);
        let end = Instant::now();

        let dec = self.ck.decrypt_ascii(&result);

        println!(
            "\n\x1b[1mReplacen:\x1b[0m\n\
            \x1b[1;32m--------------------------------\x1b[0m\n\
            \x1b[1;32;1mString: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mFrom (clear): \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mTo: \x1b[0m\x1b[0;33m{:?}\x1b[0m\n\
            \x1b[1;32;1mTimes (encrypted): \x1b[0m{}\n\
            \x1b[1;32;1mClear API Result: \x1b[0m{:?}\n\
            \x1b[1;32;1mT-fhe API Result: \x1b[0m{:?}\n\
            \x1b[1;34mExecution Time: \x1b[0m{:?}\n\
            \x1b[1;32m--------------------------------\x1b[0m",
            str,
            pat,
            to,
            n,
            expected,
            dec,
            end.duration_since(start),
        );
        assert_eq!(dec, expected);
    }
}