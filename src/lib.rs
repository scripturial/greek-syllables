use unicode_segmentation::UnicodeSegmentation;
use Accent::*;
use Breathing::*;

// Split a sequence of unicode Greek characters into syllables.
// Characters can be accented. Characters can be composed as NFD or NFC.
pub fn syllables<'a>(word: &'a str) -> Vec<&'a str> {
    let mut syllables = Vec::<&'a str>::new();
    let mut state = Syllable::Ending;
    let mut end: usize = word.len();
    let mut last: usize = 0;
    let mut prev: char = 0 as char;
    let mut prev_prev: char = 0 as char;
    for (i, element) in UnicodeSegmentation::grapheme_indices(word, true).rev() {
        let (c, _, vowel, _, _, diaeresis) = categorise(element);
        //println!("mode: {:?}, {}", state, c);
        match state {
            Syllable::Ending => {
                // Consume consonants until we consume a vowel.
                if vowel {
                    if diaeresis {
                        syllables.push(&word[i..end]);
                        end = i;
                    }
                    state = Syllable::Starting;
                }
            }
            Syllable::Starting => {
                // Complete syllable with dipthong or preceeding consonant.
                if is_dipthong(c, prev) {
                    // Keep reading
                } else if vowel {
                    syllables.push(&word[last..end]);
                    end = last;
                } else {
                    state = Syllable::Restarting;
                }
            }
            Syllable::Restarting => {
                // Probably start of a new syllable, but some
                // consonance can be squshed on to the start.
                if vowel {
                    syllables.push(&word[last..end]);
                    end = last;
                    state = Syllable::Starting;
                } else if c == 'σ' && prev == 'τ' && prev_prev == 'ρ' {
                    syllables.push(&word[i..end]);
                    end = i;
                    state = Syllable::Ending;
                } else if joinable_consonant(c, prev) {
                    //println!("can join {} {}", c, prev);
                    prev_prev = prev;
                } else {
                    syllables.push(&word[last..end]);
                    end = last;
                    state = Syllable::Ending;
                }
            }
        }
        prev = c;
        last = i;
    }
    if end != 0 {
        syllables.push(&word[0..end]);
    }

    syllables.reverse();
    return syllables;
}

#[derive(Debug)]
enum Syllable {
    Ending,
    Starting,
    Restarting,
}

#[inline]
fn is_dipthong(a: char, b: char) -> bool {
    //println!("is dipthong {} {}", a, b);
    match (a, b) {
        ('α', 'ι') => true,
        ('ε', 'ι') => true,
        ('ο', 'ι') => true,
        ('υ', 'ι') => true,
        ('α', 'υ') => true,
        ('ε', 'υ') => true,
        ('ο', 'υ') => true,
        ('η', 'υ') => true,
        _ => false,
    }
}

#[inline]
fn joinable_consonant(a: char, b: char) -> bool {
    match (a, b) {
        ('β', 'δ') => true,
        ('β', 'λ') => true,
        ('β', 'ρ') => true,
        ('γ', 'λ') => true,
        ('γ', 'ν') => true,
        ('γ', 'ρ') => true,
        ('δ', 'ρ') => true,
        ('θ', 'λ') => true,
        ('θ', 'ν') => true,
        ('θ', 'ρ') => true,
        ('κ', 'λ') => true,
        ('κ', 'ν') => true,
        ('κ', 'ρ') => true,
        ('κ', 'τ') => true,
        ('μ', 'ν') => true,
        ('π', 'λ') => true,
        ('π', 'ν') => true,
        ('π', 'ρ') => true,
        ('π', 'τ') => true,
        ('σ', 'β') => true,
        ('σ', 'θ') => true,
        ('σ', 'κ') => true,
        ('σ', 'μ') => true,
        ('σ', 'π') => true,
        ('σ', 'τ') => true,
        ('σ', 'φ') => true,
        ('σ', 'χ') => true,
        ('τ', 'ρ') => true,
        ('φ', 'θ') => true,
        ('φ', 'λ') => true,
        ('φ', 'ρ') => true,
        ('χ', 'λ') => true,
        ('χ', 'ρ') => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
pub enum Accent {
    Acute,
    Circumflex,
    Grave,
    Unaccented,
}

#[derive(Debug, PartialEq)]
pub enum Breathing {
    Rough,
    Smooth,
    None,
}

#[inline]
fn categorise(c: &str) -> (char, &'static str, bool, Breathing, Accent, bool) {
    //    let (a, b, k, d, e) = x_categorise(c);
    //    println!("{} -> {} {} {} {:?} {:?}", c, a, b, k, d, e);
    //    (a, b, k, d, e)
    //}
    //fn x_categorise(c: &str) -> (char, &'static str, bool, Accent, Accent) {
    match c {
        "α" => {
            return ('α', "a", true, None, Unaccented, false);
        }
        "ε" => {
            return ('ε', "e", true, None, Unaccented, false);
        }
        "η" => {
            return ('η', "e", true, None, Unaccented, false);
        }
        "ι" => {
            return ('ι', "i", true, None, Unaccented, false);
        }
        "ο" => {
            return ('ο', "o", true, None, Unaccented, false);
        }
        "υ" => {
            return ('υ', "u", true, None, Unaccented, false);
        }
        "ω" => {
            return ('ω', "o", true, None, Unaccented, false);
        }
        "ϋ" => {
            return ('υ', "υ", true, None, Unaccented, true);
        }
        "ϊ" => {
            return ('ι', "i", true, None, Unaccented, true);
        }
        "ᾶ" => {
            return ('ᾶ', "a", true, None, Circumflex, false);
        }
        "ῆ" => {
            return ('ῆ', "e", true, None, Circumflex, false);
        }
        "ῖ" => {
            return ('ῖ', "i", true, None, Circumflex, false);
        }
        "ῶ" => {
            return ('ῶ', "o", true, None, Circumflex, false);
        }
        "ά" | "α\u{301}" => {
            return ('α', "a", true, Smooth, Acute, false);
        }
        "έ" | "ε\u{301}" => {
            return ('ε', "e", true, Smooth, Acute, false);
        }
        "ή" | "η\u{301}" => {
            return ('η', "e", true, Smooth, Acute, false);
        }
        "ί" | "ι\u{301}" => {
            return ('ι', "i", true, Smooth, Acute, false);
        }
        "ό" | "ο\u{301}" => {
            return ('ο', "o", true, Smooth, Acute, false);
        }
        "ύ" | "υ\u{301}" => {
            return ('υ', "u", true, Smooth, Acute, false);
        }
        "ώ" | "ω\u{301}" => {
            return ('ω', "o", true, Smooth, Acute, false);
        }
        "ὰ" | "α\u{300}" => {
            return ('α', "a", true, Smooth, Grave, false);
        }
        "ὲ" | "ε\u{300}" => {
            return ('ε', "e", true, Smooth, Grave, false);
        }
        "ὴ" | "η\u{300}" => {
            return ('η', "e", true, Smooth, Grave, false);
        }
        "ὶ" | "ι\u{300}" => {
            return ('ι', "i", true, Smooth, Grave, false);
        }
        "ὸ" | "ο\u{300}" => {
            return ('ο', "o", true, Smooth, Grave, false);
        }
        "ὺ" | "υ\u{300}" => {
            return ('υ', "u", true, Smooth, Grave, false);
        }
        "ὼ" | "ω\u{300}" => {
            return ('ω', "o", true, Smooth, Grave, false);
        }
        "ἀ" | "α\u{313}" => {
            return ('α', "a", true, Smooth, Unaccented, false);
        }
        "ἐ" | "ε\u{313}" => {
            return ('ε', "e", true, Smooth, Unaccented, false);
        }
        "ἠ" | "η\u{313}" => {
            return ('η', "e", true, Smooth, Unaccented, false);
        }
        "ἰ" | "ι\u{313}" => {
            return ('ι', "i", true, Smooth, Unaccented, false);
        }
        "ὀ" | "ο\u{313}" => {
            return ('ο', "o", true, Smooth, Unaccented, false);
        }
        "ὐ" | "υ\u{313}" => {
            return ('υ', "u", true, Smooth, Unaccented, false);
        }
        "ὠ" | "ω\u{313}" => {
            return ('ω', "o", true, Smooth, Unaccented, false);
        }
        "ἁ" => {
            return ('α', "a", true, Rough, Unaccented, false);
        }
        "ἑ" => {
            return ('ε', "e", true, Rough, Unaccented, false);
        }
        "ἡ" => {
            return ('η', "e", true, Rough, Unaccented, false);
        }
        "ἱ" => {
            return ('ι', "i", true, Rough, Unaccented, false);
        }
        "ὁ" => {
            return ('ο', "i", true, Rough, Unaccented, false);
        }
        "ὑ" => {
            return ('υ', "u", true, Rough, Unaccented, false);
        }
        "ὡ" => {
            return ('ω', "o", true, Rough, Unaccented, false);
        }
        "ἄ" => {
            return ('α', "a", true, Smooth, Acute, false);
        }
        "ἔ" => {
            return ('ε', "e", true, Smooth, Acute, false);
        }
        "ἤ" => {
            return ('η', "e", true, Smooth, Acute, false);
        }
        "ἴ" => {
            return ('ι', "i", true, Smooth, Acute, false);
        }
        "ὄ" => {
            return ('ο', "o", true, Smooth, Acute, false);
        }
        "ὔ" => {
            return ('υ', "u", true, Smooth, Acute, false);
        }
        "ὤ" => {
            return ('ω', "o", true, Smooth, Acute, false);
        }
        "ἅ" | "α\u{301}\u{314}" | "α\u{314}\u{301}" => {
            return ('α', "a", true, Rough, Acute, false);
        }
        "ἕ" | "ε\u{301}\u{314}" | "ε\u{314}\u{301}" => {
            return ('ε', "e", true, Rough, Acute, false);
        }
        "ἥ" | "η\u{301}\u{314}" | "η\u{314}\u{301}" => {
            return ('η', "e", true, Rough, Acute, false);
        }
        "ἵ" | "ι\u{301}\u{314}" | "ι\u{314}\u{301}" => {
            return ('ι', "i", true, Rough, Acute, false);
        }
        "ὅ" | "ο\u{301}\u{314}" | "ο\u{314}\u{301}" => {
            return ('ο', "o", true, Rough, Acute, false);
        }
        "ὕ" | "υ\u{301}\u{314}" | "υ\u{314}\u{301}" => {
            return ('υ', "u", true, Rough, Acute, false);
        }
        "ὥ" | "ω\u{301}\u{314}" | "ω\u{314}\u{301}" => {
            return ('ω', "o", true, Rough, Acute, false);
        }
        "ἂ" => {
            return ('α', "a", true, Smooth, Grave, false);
        }
        "ἒ" => {
            return ('ε', "e", true, Smooth, Grave, false);
        }
        "ἢ" => {
            return ('η', "e", true, Smooth, Grave, false);
        }
        "ἲ" => {
            return ('ι', "i", true, Smooth, Grave, false);
        }
        "ὂ" => {
            return ('ο', "o", true, Smooth, Grave, false);
        }
        "ὒ" => {
            return ('υ', "u", true, Smooth, Grave, false);
        }
        "ὢ" => {
            return ('ω', "o", true, Smooth, Grave, false);
        }
        "ἃ" => {
            return ('α', "a", true, Rough, Grave, false);
        }
        "ἓ" => {
            return ('ε', "e", true, Rough, Grave, false);
        }
        "ἣ" => {
            return ('η', "e", true, Rough, Grave, false);
        }
        "ἳ" => {
            return ('ι', "i", true, Rough, Grave, false);
        }
        "ὃ" => {
            return ('ο', "o", true, Rough, Grave, false);
        }
        "ὓ" => {
            return ('υ', "u", true, Rough, Grave, false);
        }
        "ὣ" => {
            return ('ω', "o", true, Rough, Grave, false);
        }
        "β" => {
            return ('β', "b", false, None, Unaccented, false);
        }
        "γ" => {
            return ('γ', "g", false, None, Unaccented, false);
        }
        "δ" => {
            return ('δ', "d", false, None, Unaccented, false);
        }
        "ζ" => {
            return ('ζ', "z", false, None, Unaccented, false);
        }
        "θ" => {
            return ('θ', "th", false, None, Unaccented, false);
        }
        "κ" => {
            return ('κ', "k", false, None, Unaccented, false);
        }
        "λ" => {
            return ('λ', "l", false, None, Unaccented, false);
        }
        "μ" => {
            return ('μ', "m", false, None, Unaccented, false);
        }
        "ν" => {
            return ('ν', "n", false, None, Unaccented, false);
        }
        "ξ" => {
            return ('ξ', "x", false, None, Unaccented, false);
        }
        "π" => {
            return ('π', "p", false, None, Unaccented, false);
        }
        "ρ" => {
            return ('ρ', "r", false, None, Unaccented, false);
        }
        "σ" => {
            return ('σ', "s", false, None, Unaccented, false);
        }
        "τ" => {
            return ('τ', "t", false, None, Unaccented, false);
        }
        "φ" => {
            return ('φ', "f", false, None, Unaccented, false);
        }
        "χ" => {
            return ('χ', "ch", false, None, Unaccented, false);
        }
        "ψ" => {
            return ('ψ', "ps", false, None, Unaccented, false);
        }
        _ => (0 as char, "", false, None, Unaccented, false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorise() {
        let (gc, c, vowel, breathing, accent, diaeresis) = categorise("α");
        assert_eq!(c, "a");
        assert_eq!(gc, 'α');
        assert_eq!(vowel, true);
        assert_eq!(breathing, None);
        assert_eq!(accent, Unaccented);
        assert_eq!(diaeresis, false);

        let (gc, c, vowel, breathing, accent, _) = categorise("β");
        assert_eq!(c, "b");
        assert_eq!(gc, 'β');
        assert_eq!(vowel, false);
        assert_eq!(breathing, None);
        assert_eq!(accent, Unaccented);

        let (gc, c, vowel, breathing, accent, _) = categorise("ὔ");
        assert_eq!(c, "u");
        assert_eq!(gc, 'υ');
        assert_eq!(vowel, true);
        assert_eq!(breathing, Smooth);
        assert_eq!(accent, Acute);
    }

    #[test]
    fn test_basic_words() {
        // NFC
        assert_eq!(syllables("αα"), ["α", "α"]);
        assert_eq!(syllables("καα"), ["κα", "α"]);
        assert_eq!(syllables("αλα"), ["α", "λα"]);
        assert_eq!(syllables("ἄμα"), ["ἄ", "μα"]);
        assert_eq!(syllables("ἀλά"), ["ἀ", "λά"]);
        assert_eq!(syllables("ἀλὰ"), ["ἀ", "λὰ"]);
        assert_eq!(syllables("χριστος"), ["χρι", "στος"]);
        assert_eq!(syllables("χρίστος"), ["χρί", "στος"]);
        assert_eq!(syllables("περιπα"), ["πε", "ρι", "πα"]);
        assert_eq!(syllables("τραγος"), ["τρα", "γος"]);
        assert_eq!(syllables("στρατιοτης"), ["στρα", "τι", "ο", "της"]);
        assert_eq!(syllables("πιστευω"), ["πι", "στευ", "ω"]);
        assert_eq!(syllables("γυναικός"), ["γυ", "ναι", "κός"]);
        assert_eq!(syllables("φυω"), ["φυ", "ω"]);
        assert_eq!(syllables("σσσ"), ["σσσ"]);
        assert_eq!(syllables("μωϋσῆν"), ["μω", "ϋ", "σῆν"]);

        // NFD
        assert_eq!(syllables("ἀετός"), ["ἀ", "ε", "τός"]);
        assert_eq!(syllables("ἀετὸν"), ["ἀ", "ε", "τὸν"]);
        assert_eq!(syllables("ὥσπερ"), ["ὥ", "σπερ"]); // TODO: Is this correct?
        assert_eq!(syllables("ἔχει"), ["ἔ", "χει"]);
    }
}
