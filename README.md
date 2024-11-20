## greek-syllables

This is an experimental Zero copy Ancient Greek word syllabification library.

    use greeksyllables::syllables;
    let syllables = syllables("στρατιοτης"); // ["στρα", "τι", "ο", "της"]

Each syllable has the lifetime of the input string provded. 

This library is not yet fully tested and not guaranteed to be fully
functional. Use at your own risk. Read the test code for more details
on usage.
