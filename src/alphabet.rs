use std::{collections::HashMap, sync::OnceLock};

use clap::ValueEnum;

static ALPHABET_LOW_FREQUENCY: OnceLock<HashMap<char, char>> = OnceLock::new();
static ALPHABET_HIGH_FREQUENCY: OnceLock<HashMap<char, char>> = OnceLock::new();

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Frequency {
    Low,
    High,
}

fn load() {
    let _ = ALPHABET_LOW_FREQUENCY.get_or_init(|| {
        let mut m = HashMap::<char, char>::new();
        m.insert('a', '4');
        m.insert('b', '8');
        m.insert('o', '0');
        m.insert('e', '3');
        m.insert('i', '1');
        m.insert('s', '5');
        m
    });

    let _ = ALPHABET_HIGH_FREQUENCY.get_or_init(|| {
        let mut m = HashMap::<char, char>::new();
        m.insert('a', '4');
        m.insert('b', '8');
        m.insert('c', '[');
        m.insert('d', ')');
        m.insert('e', '3');
        m.insert('g', '6');
        m.insert('h', '#');
        m.insert('i', '1');
        m.insert('j', ']');
        m.insert('k', 'x');
        m.insert('l', '1');
        m.insert('n', '~');
        m.insert('o', '0');
        m.insert('p', '?');
        m.insert('q', '9');
        m.insert('r', '2');
        m.insert('s', '5');
        m.insert('t', '+');
        m.insert('u', 'v');
        m.insert('x', '*');
        m.insert('y', 'j');
        m.insert('z', '2');
        m
    });
}

fn encode(alphabet: &HashMap<char, char>, data: &str) -> String {
    let result = data
        .chars()
        .map(|c| {
            if let Some(character) = alphabet.get(&c) {
                *character
            } else {
                c
            }
        })
        .collect::<String>();
    result
}

pub fn encodestr(frequency: Frequency, data: &str) -> String {
    load();
    let result = match frequency {
        Frequency::Low => encode(ALPHABET_LOW_FREQUENCY.get().unwrap(), data),
        Frequency::High => encode(ALPHABET_HIGH_FREQUENCY.get().unwrap(), data),
    };
    result
}

#[cfg(test)]
mod tests {
    use super::encodestr;

    #[test]
    fn test_encode_str_data() {
        let data = "the quick brown fox jumps over the lazy dog";
        let low_frequency = "th3 qu1ck 8r0wn f0x jump5 0v3r th3 l4zy d0g";
        let full_frequency = "+#3 9v1[x 820w~ f0* ]vm?5 0v32 +#3 142j )06";
        let result = encodestr(super::Frequency::Low, data);
        assert_eq!(low_frequency, result);
        let result = encodestr(super::Frequency::High, data);
        assert_eq!(full_frequency, result);
    }
}
