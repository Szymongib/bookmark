use crate::types::URLRecord;

pub trait Filter {
    fn matches(&self, record: &URLRecord) -> bool;
}

pub struct NoopFilter {}

impl NoopFilter {
    pub fn new() -> NoopFilter {
        NoopFilter {}
    }
}

impl Filter for NoopFilter {
    fn matches(&self, record: &URLRecord) -> bool {
        true
    }
}

pub struct FilterSet {
    filters: Vec<URLFilter>,
}

// TODO: consider refactoring to use only as a function
// TODO: add builder for combined filters?

impl FilterSet {
    pub fn new_combined_filter(phrase: &str) -> FilterSet {
        return FilterSet {
            filters: vec![
                URLFilter::new_name_filter(phrase),
                URLFilter::new_url_filter(phrase),
                URLFilter::new_group_filter(phrase),
                URLFilter::new_tag_filter(phrase),
            ],
        };
    }
}

impl Filter for FilterSet {
    fn matches(&self, record: &URLRecord) -> bool {
        for f in &self.filters {
            if f.matches(record) {
                return true;
            }
        }
        return false;
    }
}

// TODO: Tag filter, Group filter etc.
// TODO: combine filters

pub struct GroupFilter {
    group: String,
}

impl Filter for GroupFilter {
    fn matches(&self, record: &URLRecord) -> bool {
        record.group == self.group
    }
}

impl GroupFilter {
    pub fn new(group: &str) -> GroupFilter {
        GroupFilter{group: group.to_string()}
    }
}

// TODO: support multiple tags?
pub struct TagsFilter {
    tags: Vec<String>,
}

impl Filter for TagsFilter {
    fn matches(&self, record: &URLRecord) -> bool {
        for t in &self.tags {
            if record.tags.contains_key(t) {
                return true
            }
        }
        return false
    }
}

impl TagsFilter {
    pub fn new(tags: Vec<&str>) -> TagsFilter {
        TagsFilter {tags: tags.iter().map(|t| {t.to_string()}).collect()}
    }
}

enum SearchElement {
    Name,
    URL,
    Group,
    Tag,
}

pub struct URLFilter {
    phrase: String,
    element: SearchElement,
}

impl URLFilter {
    pub fn new_name_filter(phrase: &str) -> URLFilter {
        URLFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::Name,
        }
    }

    pub fn new_url_filter(phrase: &str) -> URLFilter {
        URLFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::URL,
        }
    }

    pub fn new_group_filter(phrase: &str) -> URLFilter {
        URLFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::Group,
        }
    }

    pub fn new_tag_filter(phrase: &str) -> URLFilter {
        URLFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::Tag,
        }
    }

    pub fn matches(&self, record: &URLRecord) -> bool {
        return match &self.element {
            SearchElement::Name => record.name.to_lowercase().contains(&self.phrase),
            SearchElement::URL => record.url.to_lowercase().contains(&self.phrase),
            SearchElement::Group => record.group.to_lowercase().contains(&self.phrase),
            SearchElement::Tag => {
                for (t, _) in &record.tags {
                    if t.to_lowercase().contains(&self.phrase) {
                        return true;
                    }
                }
                return false;
            }
        };
    }
}

// TODO: more complex tests with only some types of filters

#[cfg(test)]
mod test {
    use crate::record_filter::{FilterSet, Filter};
    use crate::types::URLRecord;

    #[test]
    fn filter_set_test() {
        let test_set = vec![
            URLRecord::new(
                "http://urlAbcd.com",
                "first url",
                "default",
                vec!["pop", "with space"],
            ),
            URLRecord::new("http://another.com", "second ABCD", "default", vec!["pop"]),
            URLRecord::new(
                "http://another.com",
                "third with space",
                "group-abcd",
                vec!["pop"],
            ),
            URLRecord::new(
                "http://another.com",
                "fourth",
                "default",
                vec!["pop", "tag-abcd"],
            ),
            URLRecord::new(
                "http://acbd.com",
                "fifth with space",
                "default",
                vec!["pop", "another"],
            ),
        ];

        struct TestCase {
            phrase: String,
            matches: Vec<bool>,
        }

        let test_cases = vec![
            TestCase {
                phrase: "abcd".to_string(),
                matches: vec![true, true, true, true, false],
            },
            TestCase {
                phrase: "Another".to_string(),
                matches: vec![false, true, true, true, true],
            },
            TestCase {
                phrase: "pop".to_string(),
                matches: vec![true, true, true, true, true],
            },
            TestCase {
                phrase: "third".to_string(),
                matches: vec![false, false, true, false, false],
            },
            TestCase {
                phrase: "with space".to_string(),
                matches: vec![true, false, true, false, true],
            },
            TestCase {
                phrase: "non existent".to_string(),
                matches: vec![false, false, false, false, false],
            },
        ];

        for test in test_cases {
            println!("Phrase: {}", test.phrase);

            let combined_filter: FilterSet = FilterSet::new_combined_filter(test.phrase.as_str());

            for i in 0..test_set.len() {
                println!("URL: {}", &test_set[i]);
                assert_eq!(combined_filter.matches(&test_set[i]), test.matches[i])
            }
        }
    }
}
