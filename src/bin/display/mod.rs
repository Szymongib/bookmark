use std::collections::HashMap;
use bookmark_lib::types::URLRecord;

pub(crate) fn display_urls(urls: Vec<URLRecord>) {
    let (name_len, url_len, group_len, tags_len) = get_max_lengths(&urls);
    let id_len = if urls.len() > 0 {
        urls[0].id.len()  // Ids have uniform length
    } else {
        0
    };

    println!("{}", header(id_len, name_len, url_len, group_len, tags_len));
    println!();

    for u in urls {
        println!("{}   {}   {}   {}   {}",
                 pad(u.id.clone(), id_len),
                 pad(u.name.clone(), name_len),
                 pad(u.url.clone(), url_len),
                 pad(u.group.clone(), group_len),
                 pad(u.tags_as_string(), tags_len))
    }
}

fn header(id_len: usize, name_len: usize, url_len: usize, group_len: usize, tags_len: usize) -> String {
    let id = pad("Id".to_string(), id_len);
    let name = pad("Name".to_string(), name_len);
    let url = pad("URL".to_string(), url_len);
    let group = pad("Group".to_string(), group_len);
    let tags = pad("Tags".to_string(), tags_len);

    format!("{}   {}   {}   {}   {}", id, name, url, group, tags)
}

fn pad(s: String, len: usize) -> String {
    let mut s = s.clone();
    for _ in 0..(len - s.len()) {
        s.push(' ');
    }
    return s
}

/// Returns max length of Name, URL, Group, Tags
fn get_max_lengths(urls: &Vec<URLRecord>) -> (usize, usize, usize, usize) {
    let mut max_len: [usize; 4] = [0,0,0,0];

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
        if u.tags_as_string().len() > max_len[3] {
            max_len[3] = u.name.len()
        }
    }

    return (max_len[0], max_len[1], max_len[2], max_len[3])
}
//
// /// Return combined length of tags displayed in format: 'tag1, tag2, tag3'
// fn tags_len(tags: &HashMap<String, bool>) -> usize {
//     let mut sum = 0;
//     for (t, _) in tags {
//         sum += t.len();
//     }
//     sum+= (tags.len() - 1) * 2; // add length of ", " for every tag more than one
//     return sum
// }

