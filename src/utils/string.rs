use regex::Regex;

/// Converts a string to PascalCase.
///
/// Example Input : This is a test string
///
/// Example Output : ThisIsATestString
pub fn to_pascal_case(input: &str, is_trim_spaces: bool) -> String {
    let re = Regex::new(r"\b[a-z]").unwrap();

    let result = re.replace_all(input, |caps: &regex::Captures| {
        caps.get(0).unwrap().as_str().to_uppercase()
    });

    if is_trim_spaces {
        result.replace(" ", "")
    } else {
        result.to_string()
    }
}

/// Converts a string to snake_case.
///
/// Example Input : This is a test string
///
/// Example Output : this_is_a_test_string
pub fn to_snake_case(input: &str) -> String {
    let re = Regex::new(r"(?P<first>[A-Z])(?P<rest>[a-z]+)").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        format!("{}{}", caps["first"].to_lowercase(), &caps["rest"])
    })
    .replace(" ", "_")
}

/// Converts a string to kebab-case.
///
/// Example Input : This is a test string
///
/// Example Out : this-is-a-test-string
pub fn to_kebab_case(input: &str) -> String {
    let re = Regex::new(r"(?P<first>[A-Z])(?P<rest>[a-z]+)").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        format!("{}{}", caps["first"].to_lowercase(), &caps["rest"])
    })
    .replace(" ", "-")
}

/// Converts a string to camelCase.
///
/// Example Input : $]v>5gb=ai|ox~:^i"hz
///
/// Example Output : ดอลลาร์ - สี่เหลี่ยม ปิด - วี เล็ก - มากกว่า - ห้า - จี เล็ก - บี เล็ก - เท่ากับ - เอ เล็ก - ไอ เล็ก - ไปร์ - โอ เล็ก - เอ็กซ์ เล็ก - ทิลเด้ - โคลอน - แครต - ไอ เล็ก - เครื่องหมายคำพูด - เอช เล็ก - แซด เล็ก
pub fn to_camel_case(input: &str, is_trim_space: bool) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut result = String::new();

    for (i, word) in words.iter().enumerate() {
        if i == 0 {
            result.push_str(&word.to_lowercase());
        } else {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_uppercase().next().unwrap_or(first));
                result.push_str(&chars.as_str().to_lowercase());
            }
        }

        if !is_trim_space && i < words.len() - 1 {
            result.push(' ');
        }
    }

    result
}

/// Converts a string to Thai pronunciation.
///
/// Example Input : &9*[oJ}KXAKU}iKTU##1
///
/// Example Output : แอนด์ - เก้า - สตาร์ - สี่เหลี่ยม เปิด - โอ เล็ก - เจ ใหญ่ - ปีกกา ปิด - เค ใหญ่ - เอ็กซ์ ใหญ่ - เอ ใหญ่ - เค ใหญ่ - ยู ใหญ่ - ปีกกา ปิด - ไอ เล็ก - เค ใหญ่ - ที ใหญ่ - ยู ใหญ่ - แฮชแท็ก - แฮชแท็ก - หนึ่ง
pub fn to_thai_pronunciation(input: &str) -> String {
    let mapping_lower = [
        ('a', "เอ เล็ก"),
        ('b', "บี เล็ก"),
        ('c', "ซี เล็ก"),
        ('d', "ดี เล็ก"),
        ('e', "อี เล็ก"),
        ('f', "เอฟ เล็ก"),
        ('g', "จี เล็ก"),
        ('h', "เอช เล็ก"),
        ('i', "ไอ เล็ก"),
        ('j', "เจ เล็ก"),
        ('k', "เค เล็ก"),
        ('l', "แอล เล็ก"),
        ('m', "เอ็ม เล็ก"),
        ('n', "เอ็น เล็ก"),
        ('o', "โอ เล็ก"),
        ('p', "พี เล็ก"),
        ('q', "คิว เล็ก"),
        ('r', "อาร์ เล็ก"),
        ('s', "เอส เล็ก"),
        ('t', "ที เล็ก"),
        ('u', "ยู เล็ก"),
        ('v', "วี เล็ก"),
        ('w', "ดับเบิลยู เล็ก"),
        ('x', "เอ็กซ์ เล็ก"),
        ('y', "วาย เล็ก"),
        ('z', "แซด เล็ก"),
    ];

    let mapping_upper = [
        ('A', "เอ ใหญ่"),
        ('B', "บี ใหญ่"),
        ('C', "ซี ใหญ่"),
        ('D', "ดี ใหญ่"),
        ('E', "อี ใหญ่"),
        ('F', "เอฟ ใหญ่"),
        ('G', "จี ใหญ่"),
        ('H', "เอช ใหญ่"),
        ('I', "ไอ ใหญ่"),
        ('J', "เจ ใหญ่"),
        ('K', "เค ใหญ่"),
        ('L', "แอล ใหญ่"),
        ('M', "เอ็ม ใหญ่"),
        ('N', "เอ็น ใหญ่"),
        ('O', "โอ ใหญ่"),
        ('P', "พี ใหญ่"),
        ('Q', "คิว ใหญ่"),
        ('R', "อาร์ ใหญ่"),
        ('S', "เอส ใหญ่"),
        ('T', "ที ใหญ่"),
        ('U', "ยู ใหญ่"),
        ('V', "วี ใหญ่"),
        ('W', "ดับเบิลยู ใหญ่"),
        ('X', "เอ็กซ์ ใหญ่"),
        ('Y', "วาย ใหญ่"),
        ('Z', "แซด ใหญ่"),
    ];

    let mapping_numbers = [
        ('0', "ศูนย์"),
        ('1', "หนึ่ง"),
        ('2', "สอง"),
        ('3', "สาม"),
        ('4', "สี่"),
        ('5', "ห้า"),
        ('6', "หก"),
        ('7', "เจ็ด"),
        ('8', "แปด"),
        ('9', "เก้า"),
    ];

    let mapping_special = [
        (' ', "เว้นวรรค"),
        ('.', "จุด"),
        (',', "คอมม่า"),
        ('-', "ขีดกลาง"),
        ('_', "ขีดล่าง"),
        ('/', "สเลช"),
        ('\\', "แบ็คสแลช"),
        ('|', "ไปร์"),
        ('@', "แอท"),
        ('#', "แฮชแท็ก"),
        ('$', "ดอลลาร์"),
        ('%', "เปอร์เซ็นต์"),
        ('^', "แครต"),
        ('*', "สตาร์"),
        ('+', "บวก"),
        ('=', "เท่ากับ"),
        ('!', "ตกใจ"),
        ('?', "เควสชั่นมาร์ค"),
        ('<', "น้อยกว่า"),
        ('>', "มากกว่า"),
        ('`', "แบ็คทิก"),
        ('~', "ทิลเด้"),
        ('"', "เครื่องหมายคำพูด"),
        ('\'', "โครต"),
        ('\"', "ดับเบิลโครต"),
        (';', "เซมิโคลอน"),
        (':', "โคลอน"),
        ('(', "วงเล็บ เปิด"),
        (')', "วงเล็บ ปิด"),
        ('[', "สี่เหลี่ยม เปิด"),
        (']', "สี่เหลี่ยม ปิด"),
        ('{', "ปีกกา เปิด"),
        ('}', "ปีกกา ปิด"),
        ('&', "แอนด์"),
    ];

    let mapping_all = [
        mapping_lower.as_ref(),
        mapping_upper.as_ref(),
        mapping_numbers.as_ref(),
        mapping_special.as_ref(),
    ]
    .concat();

    input
        .chars()
        .map(|c| {
            mapping_all
                .iter()
                .find(|&&(ch, _)| ch == c)
                .map(|&(_, desc)| desc.to_string())
                .unwrap_or_else(|| c.to_string())
        })
        .collect::<Vec<String>>()
        .join(" - ")
}
