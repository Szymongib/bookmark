use bookmark_lib::types::URLRecord;

pub(crate) fn display_urls(urls: Vec<URLRecord>) {
    println!("{}", display_str(urls))
}

fn display_str(urls: Vec<URLRecord>) -> String {
    let (name_len, url_len, group_len, tags_len) = get_max_lengths(&urls);
    let id_len = if !urls.is_empty() {
        urls[0].id.len() // Ids have uniform length
    } else {
        0
    };

    let mut out = header(id_len, name_len, url_len, group_len, tags_len);
    out.push('\n');

    for u in urls {
        out.push_str(&format!(
            "\n{}   {}   {}   {}   {}",
            pad(u.id.clone(), id_len),
            pad(u.name.clone(), name_len),
            pad(u.url.clone(), url_len),
            pad(u.group.clone(), group_len),
            pad(u.tags_as_string(), tags_len)
        ))
    }

    out
}

fn header(
    id_len: usize,
    name_len: usize,
    url_len: usize,
    group_len: usize,
    tags_len: usize,
) -> String {
    let id = pad("Id".to_string(), id_len);
    let name = pad("Name".to_string(), name_len);
    let url = pad("URL".to_string(), url_len);
    let group = pad("Group".to_string(), group_len);
    let tags = pad("Tags".to_string(), tags_len);

    format!("{}   {}   {}   {}   {}", id, name, url, group, tags)
}

fn pad(s: String, len: usize) -> String {
    let mut s = s;

    let pad_count = if len >= s.len() { len - s.len() } else { 0 };

    for _ in 0..pad_count {
        s.push(' ');
    }
    s
}

/// Returns max length of Name, URL, Group, Tags
fn get_max_lengths(urls: &[URLRecord]) -> (usize, usize, usize, usize) {
    let mut max_len: [usize; 4] = [4, 3, 5, 0];

    for u in urls {
        if u.name.len() > max_len[0] {
            max_len[0] = u.name.len()
        }
        if u.url.len() > max_len[1] {
            max_len[1] = u.url.len()
        }
        if u.group.len() > max_len[2] {
            max_len[2] = u.group.len()
        }
        let tags_len = u.tags_as_string().len();
        if tags_len > max_len[3] {
            max_len[3] = tags_len
        }
    }

    (max_len[0], max_len[1], max_len[2], max_len[3])
}

#[cfg(test)]
mod test {
    use crate::display::display_str;
    use bookmark_lib::types::URLRecord;

    struct TestCase {
        description: String,
        records: Vec<URLRecord>,
        expected_lines: Vec<String>,
    }

    #[test]
    fn test_display_str() {
        let records = vec![
            URLRecord::new("https://one_long_url.com", "one_name", "one", vec!["tag"]),
            URLRecord::new(
                "two",
                "two long name wow such name",
                "two_long_group",
                vec![],
            ),
            URLRecord::new("three", "three", "three", vec![]),
            URLRecord::new(
                "four.com",
                "four mid len",
                "4",
                vec!["tag", "other-tag", "yet-one-more"],
            ),
            URLRecord::new(
                "five",
                "five",
                "five",
                vec!["just_one_but_long_tag_much_wow"],
            ),
        ];

        let single_record = URLRecord::new(
            "https://httpbin.org",
            "HTTP Bin",
            "default",
            vec!["testing"],
        );

        let test_cases = vec![
            TestCase{
                description: "Several URL records".to_string(),
                records: records.clone(),
                expected_lines: vec![
                    "Id                 Name                          URL                        Group            Tags                          ".to_string(),
                    "".to_string(),
                    format!("{}   one_name                      https://one_long_url.com   one              tag                           ", records[0].id),
                    format!("{}   two long name wow such name   two                        two_long_group                                 ", records[1].id),
                    format!("{}   three                         three                      three                                          ", records[2].id),
                    format!("{}   four mid len                  four.com                   4                other-tag, tag, yet-one-more  ", records[3].id),
                    format!("{}   five                          five                       five             just_one_but_long_tag_much_wow", records[4].id),
                ],
            },
            TestCase{
                description: "Single URL record".to_string(),
                records: vec![single_record.clone()],
                expected_lines: vec![
                    "Id                 Name       URL                   Group     Tags   ".to_string(),
                    "".to_string(),
                    format!("{}   HTTP Bin   https://httpbin.org   default   testing", single_record.id),
                ],
            },
            TestCase{
                description: "No URL records".to_string(),
                records: vec![],
                expected_lines: vec![
                    "Id   Name   URL   Group   Tags".to_string(),
                    "".to_string(),
                ],
            }
        ];

        for test in test_cases {
            println!("Test: {}", test.description);
            let display = display_str(test.records);

            let lines: Vec<&str> = display.split("\n").collect();
            for i in 0..lines.len() {
                assert_eq!(lines[i], test.expected_lines[i])
            }
        }
    }
}
