use std::collections::HashMap;

pub fn make_symbols_array() -> [String; 19] {
    [
        '{'.to_string(),
        '}'.to_string(),
        '('.to_string(),
        ')'.to_string(),
        '['.to_string(),
        ']'.to_string(),
        '.'.to_string(),
        ','.to_string(),
        ';'.to_string(),
        '+'.to_string(),
        '-'.to_string(),
        '*'.to_string(),
        '/'.to_string(),
        '&'.to_string(),
        '|'.to_string(),
        '<'.to_string(),
        '>'.to_string(),
        '='.to_string(),
        '~'.to_string(),
    ]
}

pub fn funky_symbols() -> HashMap<String, String> {
    HashMap::from([
        (String::from("<"), String::from("&lt;")),
        (String::from(">"), String::from("&gt;")),
        (String::from('"'), String::from("&quot;")),
        (String::from("&"), String::from("&amp;")),
    ])
}
