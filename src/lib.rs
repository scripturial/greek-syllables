use unicode_segmentation::UnicodeSegmentation;
use Accent::*;
use Breathing::*;

// Split a sequence of unicode Greek characters into syllables.
// Characters can be accented. Characters can be composed as NFD or NFC.
#[inline]
pub fn syllables<'a>(word: &'a str) -> Vec<&'a str> {
    let mut syllables = Vec::<&'a str>::new();
    let mut state = Syllable::Ending;
    let mut end: usize = word.len();
    let mut last: usize = end;
    let mut prev: char = 0 as char;
    let mut prev_prev: char = 0 as char;
    for (i, element) in UnicodeSegmentation::grapheme_indices(word, true).rev() {
        let (c, _, vowel, _, _, diaeresis) = categorise(element);
        if c == 0 as char {
            state = Syllable::Ending;
            if last == end {
                syllables.push(&word[i..end]);
            } else {
                syllables.push(&word[last..end]);
                if last > i {
                    syllables.push(&word[i..last]);
                }
            }
            end = i;
            last = i;
            prev = c;
            continue;
        }
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
                // Probably start of a new syllable, but some consonant
                // clusters have special rules for splitting.
                if vowel {
                    syllables.push(&word[last..end]);
                    end = last;
                    state = Syllable::Starting;
                } else if c == 'σ' && prev == 'τ' && prev_prev == 'ρ' {
                    syllables.push(&word[i..end]);
                    end = i;
                    state = Syllable::Ending;
                } else if joinable_consonant(c, prev) {
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
    match c {
        "Α" | "α" => {
            return ('α', "a", true, None, Unaccented, false);
        }
        "Ε" | "ε" => {
            return ('ε', "e", true, None, Unaccented, false);
        }
        "Η" | "η" => {
            return ('η', "e", true, None, Unaccented, false);
        }
        "Ι" | "ι" => {
            return ('ι', "i", true, None, Unaccented, false);
        }
        "Ο" | "ο" => {
            return ('ο', "o", true, None, Unaccented, false);
        }
        "Υ" | "υ" => {
            return ('υ', "u", true, None, Unaccented, false);
        }
        "Ω" | "ω" => {
            return ('ω', "o", true, None, Unaccented, false);
        }
        "Ϋ" | "ϋ" => {
            return ('υ', "υ", true, None, Unaccented, true);
        }
        "Ϊ" | "ϊ" => {
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
        "Ά" | "Α\u{301}" | "ά" | "α\u{301}" => {
            return ('α', "a", true, Smooth, Acute, false);
        }
        "Έ" | "Ε\u{301}" | "έ" | "ε\u{301}" => {
            return ('ε', "e", true, Smooth, Acute, false);
        }
        "Ή" | "Η\u{301}" | "ή" | "η\u{301}" => {
            return ('η', "e", true, Smooth, Acute, false);
        }
        "Ί" | "Ι\u{301}" | "ί" | "ι\u{301}" => {
            return ('ι', "i", true, Smooth, Acute, false);
        }
        "Ό" | "Ο\u{301}" | "ό" | "ο\u{301}" => {
            return ('ο', "o", true, Smooth, Acute, false);
        }
        "Ύ" | "Υ\u{301}" | "ύ" | "υ\u{301}" => {
            return ('υ', "u", true, Smooth, Acute, false);
        }
        "Ώ" | "Ω\u{301}" | "ώ" | "ω\u{301}" => {
            return ('ω', "o", true, Smooth, Acute, false);
        }
        "Ὰ" | "Α\u{300}" | "ὰ" | "α\u{300}" => {
            return ('α', "a", true, Smooth, Grave, false);
        }
        "Ὲ" | "Ε\u{300}" | "ὲ" | "ε\u{300}" => {
            return ('ε', "e", true, Smooth, Grave, false);
        }
        "Ὴ" | "Η\u{300}" | "ὴ" | "η\u{300}" => {
            return ('η', "e", true, Smooth, Grave, false);
        }
        "Ὶ" | "Ι\u{300}" | "ὶ" | "ι\u{300}" => {
            return ('ι', "i", true, Smooth, Grave, false);
        }
        "Ὸ" | "Ο\u{300}" | "ὸ" | "ο\u{300}" => {
            return ('ο', "o", true, Smooth, Grave, false);
        }
        "Ὺ" | "Υ\u{300}" | "ὺ" | "υ\u{300}" => {
            return ('υ', "u", true, Smooth, Grave, false);
        }
        "Ὼ" | "Ω\u{300}" | "ὼ" | "ω\u{300}" => {
            return ('ω', "o", true, Smooth, Grave, false);
        }
        "Ἀ" | "Α\u{313}" | "ἀ" | "α\u{313}" => {
            return ('α', "a", true, Smooth, Unaccented, false);
        }
        "Ἐ" | "Ε\u{313}" | "ἐ" | "ε\u{313}" => {
            return ('ε', "e", true, Smooth, Unaccented, false);
        }
        "Ἠ" | "Η\u{313}" | "ἠ" | "η\u{313}" => {
            return ('η', "e", true, Smooth, Unaccented, false);
        }
        "Ἰ" | "Ι\u{313}" | "ἰ" | "ι\u{313}" => {
            return ('ι', "i", true, Smooth, Unaccented, false);
        }
        "Ὀ" | "Ο\u{313}" | "ὀ" | "ο\u{313}" => {
            return ('ο', "o", true, Smooth, Unaccented, false);
        }
        "Υ\u{313}" | "ὐ" | "υ\u{313}" => {
            return ('υ', "u", true, Smooth, Unaccented, false);
        }
        "Ὠ" | "Ω\u{313}" | "ὠ" | "ω\u{313}" => {
            return ('ω', "o", true, Smooth, Unaccented, false);
        }
        "Ἁ" | "ἁ" => {
            return ('α', "a", true, Rough, Unaccented, false);
        }
        "Ἑ" | "ἑ" => {
            return ('ε', "e", true, Rough, Unaccented, false);
        }
        "Ἡ" | "ἡ" => {
            return ('η', "e", true, Rough, Unaccented, false);
        }
        "Ἱ" | "ἱ" => {
            return ('ι', "i", true, Rough, Unaccented, false);
        }
        "Ὁ" | "ὁ" => {
            return ('ο', "i", true, Rough, Unaccented, false);
        }
        "Ὑ" | "ὑ" => {
            return ('υ', "u", true, Rough, Unaccented, false);
        }
        "Ὡ" | "ὡ" => {
            return ('ω', "o", true, Rough, Unaccented, false);
        }
        "Ἄ" | "ἄ" => {
            return ('α', "a", true, Smooth, Acute, false);
        }
        "Ἔ" | "ἔ" | "ε\u{313}\u{301}" | "ε\u{301}\u{313}" | "Ε\u{313}\u{301}" | "Ε\u{301}\u{313}" => {
            return ('ε', "e", true, Smooth, Acute, false);
        }
        "Ἤ" | "ἤ" | "η\u{313}\u{301}" | "η\u{301}\u{313}" | "Η\u{313}\u{301}" | "Η\u{301}\u{313}" => {
            return ('η', "e", true, Smooth, Acute, false);
        }
        "Ἴ" | "ἴ" | "ι\u{313}\u{301}" | "ι\u{301}\u{313}" | "Ι\u{313}\u{301}" | "Ι\u{301}\u{313}" => {
            return ('ι', "i", true, Smooth, Acute, false);
        }
        "Ὄ" | "ὄ" | "ο\u{313}\u{301}" | "ο\u{301}\u{313}" | "Ο\u{313}\u{301}" | "Ο\u{301}\u{313}" => {
            return ('ο', "o", true, Smooth, Acute, false);
        }
        "ὔ" | "υ\u{313}\u{301}" | "υ\u{301}\u{313}" => {
            return ('υ', "u", true, Smooth, Acute, false);
        }
        "Ὤ" | "ὤ" | "Ω\u{313}\u{301}" | "Ω\u{301}\u{313}" | "ω\u{313}\u{301}" | "ω\u{301}\u{313}" => {
            return ('ω', "o", true, Smooth, Acute, false);
        }
        "ἅ" | "α\u{301}\u{314}" | "α\u{314}\u{301}" | "Ἅ" | "Α\u{301}\u{314}" | "Α\u{314}\u{301}" => {
            return ('α', "a", true, Rough, Acute, false);
        }
        "ἕ" | "ε\u{301}\u{314}" | "ε\u{314}\u{301}" | "Ἕ" | "Ε\u{301}\u{314}" | "Ε\u{314}\u{301}" => {
            return ('ε', "e", true, Rough, Acute, false);
        }
        "ἥ" | "η\u{301}\u{314}" | "η\u{314}\u{301}" | "Ἥ" | "Η\u{301}\u{314}" | "Η\u{314}\u{301}" => {
            return ('η', "e", true, Rough, Acute, false);
        }
        "ἵ" | "ι\u{301}\u{314}" | "ι\u{314}\u{301}" | "Ἵ" | "Ι\u{301}\u{314}" | "Ι\u{314}\u{301}" => {
            return ('ι', "i", true, Rough, Acute, false);
        }
        "ὅ" | "ο\u{301}\u{314}" | "ο\u{314}\u{301}" | "Ὅ" | "Ο\u{301}\u{314}" | "Ο\u{314}\u{301}" => {
            return ('ο', "o", true, Rough, Acute, false);
        }
        "ὕ" | "υ\u{301}\u{314}" | "υ\u{314}\u{301}" | "Ὕ" | "Υ\u{301}\u{314}" | "Υ\u{314}\u{301}" => {
            return ('υ', "u", true, Rough, Acute, false);
        }
        "ὥ" | "ω\u{301}\u{314}" | "ω\u{314}\u{301}" | "Ὥ" | "Ω\u{301}\u{314}" | "Ω\u{314}\u{301}" => {
            return ('ω', "o", true, Rough, Acute, false);
        }
        "Ἂ" | "ἂ" => {
            return ('α', "a", true, Smooth, Grave, false);
        }
        "Ἒ" | "ἒ" => {
            return ('ε', "e", true, Smooth, Grave, false);
        }
        "Ἢ" | "ἢ" => {
            return ('η', "e", true, Smooth, Grave, false);
        }
        "Ἲ" | "ἲ" => {
            return ('ι', "i", true, Smooth, Grave, false);
        }
        "Ὂ" | "ὂ" => {
            return ('ο', "o", true, Smooth, Grave, false);
        }
        "ὒ" => {
            return ('υ', "u", true, Smooth, Grave, false);
        }
        "Ὢ" | "ὢ" => {
            return ('ω', "o", true, Smooth, Grave, false);
        }
        "Ἃ" | "ἃ" => {
            return ('α', "a", true, Rough, Grave, false);
        }
        "Ἓ" | "ἓ" => {
            return ('ε', "e", true, Rough, Grave, false);
        }
        "Ἣ" | "ἣ" => {
            return ('η', "e", true, Rough, Grave, false);
        }
        "Ἳ" | "ἳ" => {
            return ('ι', "i", true, Rough, Grave, false);
        }
        "Ὃ" | "ὃ" => {
            return ('ο', "o", true, Rough, Grave, false);
        }
        "Ὓ" | "ὓ" => {
            return ('υ', "u", true, Rough, Grave, false);
        }
        "Ὣ" | "ὣ" => {
            return ('ω', "o", true, Rough, Grave, false);
        }
        "Β" | "β" => {
            return ('β', "b", false, None, Unaccented, false);
        }
        "Γ" | "γ" => {
            return ('γ', "g", false, None, Unaccented, false);
        }
        "Δ" | "δ" => {
            return ('δ', "d", false, None, Unaccented, false);
        }
        "Ζ" | "ζ" => {
            return ('ζ', "z", false, None, Unaccented, false);
        }
        "Θ" | "θ" => {
            return ('θ', "th", false, None, Unaccented, false);
        }
        "Κ" | "κ" => {
            return ('κ', "k", false, None, Unaccented, false);
        }
        "Λ" | "λ" => {
            return ('λ', "l", false, None, Unaccented, false);
        }
        "Μ" | "μ" => {
            return ('μ', "m", false, None, Unaccented, false);
        }
        "Ν" | "ν" => {
            return ('ν', "n", false, None, Unaccented, false);
        }
        "Ξ" | "ξ" => {
            return ('ξ', "x", false, None, Unaccented, false);
        }
        "Π" | "π" => {
            return ('π', "p", false, None, Unaccented, false);
        }
        "Ρ" | "ρ" => {
            return ('ρ', "r", false, None, Unaccented, false);
        }
        "Σ" | "σ" | "ς" => {
            return ('σ', "s", false, None, Unaccented, false);
        }
        "Τ" | "τ" => {
            return ('τ', "t", false, None, Unaccented, false);
        }
        "Φ" | "φ" => {
            return ('φ', "f", false, None, Unaccented, false);
        }
        "Χ" | "χ" => {
            return ('χ', "ch", false, None, Unaccented, false);
        }
        "Ψ" | "ψ" => {
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

        let (gc, c, vowel, breathing, accent, _) = categorise("Β");
        assert_eq!(c, "b");
        assert_eq!(gc, 'β');
        assert_eq!(vowel, false);
        assert_eq!(breathing, None);
        assert_eq!(accent, Unaccented);
    }

    #[test]
    fn test_invalid_characters() {
        // Invalid ascii found
        assert_eq!(syllables("σaσ"), ["σ", "a", "σ"]);
        assert_eq!(syllables("eχει"), ["e", "χει"]);
        assert_eq!(syllables("ἔχεi"), ["ἔ", "χε", "i"]);
        assert_eq!(syllables("ἔχeι"), ["ἔχ", "e", "ι"]);
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
        assert_eq!(syllables("ἈΛΆ"), ["Ἀ", "ΛΆ"]);
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
        assert_eq!(syllables("ὄσπριον"), ["ὄ", "σπρι", "ον"]);
        assert_eq!(syllables("ὁσία"), ["ὁ", "σί", "α"]);
        assert_eq!(syllables("ὅτου"), ["ὅ", "του"]);
        assert_eq!(syllables("ἥτις"), ["ἥ", "τις"]);
        assert_eq!(syllables("αἵτινες"), ["αἵ", "τι", "νες"]);
        assert_eq!(syllables("οἵτινες"), ["οἵ", "τι", "νες"]);
        assert_eq!(syllables("περιεπατήσαμεν"), ["πε", "ρι", "ε", "πα", "τή", "σα", "μεν"]);

        // NFD
        assert_eq!(syllables("ἀετός"), ["ἀ", "ε", "τός"]);
        assert_eq!(syllables("ἀετὸν"), ["ἀ", "ε", "τὸν"]);
        assert_eq!(syllables("ὥσπερ"), ["ὥ", "σπερ"]); // TODO: Is this correct?
        assert_eq!(syllables("ἔχει"), ["ἔ", "χει"]);
    }
}
