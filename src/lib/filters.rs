use crate::types::URLRecord;

pub trait Filter {
    fn matches(&self, record: &URLRecord) -> bool;
    fn chain(self, filter: Box<dyn Filter>) -> Box<dyn Filter>;
}

#[derive(Default)]
pub struct NoopFilter {}

impl Filter for NoopFilter {
    fn matches(&self, _record: &URLRecord) -> bool {
        true
    }
    fn chain(self, filter: Box<dyn Filter>) -> Box<dyn Filter> {
        Box::new(FilterSet::new_combined(vec![filter]))
    }
}

/// UnorderedWordSetFilter searches for individual words in the search phrase
/// Only records containing all words will match the filter
pub struct UnorderedWordSetFilter {
    phrase: String,
}

impl Filter for UnorderedWordSetFilter {
    fn matches(&self, record: &URLRecord) -> bool {
        if self.phrase == "" {
            return true;
        }

        for p in self.phrase.split(' ').filter(|p| !p.is_empty()) {
            let word = p.to_lowercase();

            // Check if any part matches the word
            let matches = record.name.to_lowercase().contains(&word)
                || record.url.to_lowercase().contains(&word)
                || record.group.to_lowercase().contains(&word)
                || tag_matches(record, &word);

            if !matches {
                return false;
            }
        }

        true
    }

    fn chain(self, filter: Box<dyn Filter>) -> Box<dyn Filter> {
        Box::new(FilterSet::new_combined(vec![Box::new(self), filter]))
    }
}

impl UnorderedWordSetFilter {
    pub fn new(phrase: &str) -> UnorderedWordSetFilter {
        UnorderedWordSetFilter {
            phrase: phrase.to_string(),
        }
    }
}

pub struct FilterSet {
    filters: Vec<Box<dyn Filter>>,
}

// TODO: add builder for combined filters?

impl FilterSet {
    pub fn new_combined_for_phrase(phrase: &str) -> FilterSet {
        FilterSet {
            filters: vec![
                Box::new(PhraseFilter::new_name_filter(phrase)),
                Box::new(PhraseFilter::new_url_filter(phrase)),
                Box::new(PhraseFilter::new_group_filter(phrase)),
                Box::new(PhraseFilter::new_tag_filter(phrase)),
            ],
        }
    }

    pub fn new_combined(filters: Vec<Box<dyn Filter>>) -> FilterSet {
        FilterSet { filters }
    }
}

impl Filter for FilterSet {
    fn matches(&self, record: &URLRecord) -> bool {
        for f in &self.filters {
            if f.matches(record) {
                return true;
            }
        }
        false
    }

    fn chain(self, filter: Box<dyn Filter>) -> Box<dyn Filter> {
        Box::new(FilterSet::new_combined(vec![Box::new(self), filter]))
    }
}

pub struct GroupFilter {
    group: String,
}

impl Filter for GroupFilter {
    fn matches(&self, record: &URLRecord) -> bool {
        record.group == self.group
    }
    fn chain(self, filter: Box<dyn Filter>) -> Box<dyn Filter> {
        Box::new(FilterSet::new_combined(vec![Box::new(self), filter]))
    }
}

impl GroupFilter {
    pub fn new(group: &str) -> GroupFilter {
        GroupFilter {
            group: group.to_string(),
        }
    }
}

pub struct TagsFilter {
    tags: Vec<String>,
}

impl Filter for TagsFilter {
    fn matches(&self, record: &URLRecord) -> bool {
        for t in &self.tags {
            if record.tags.contains_key(t) {
                return true;
            }
        }
        false
    }
    fn chain(self, filter: Box<dyn Filter>) -> Box<dyn Filter> {
        Box::new(FilterSet::new_combined(vec![Box::new(self), filter]))
    }
}

impl TagsFilter {
    pub fn new(tags: Vec<&str>) -> TagsFilter {
        TagsFilter {
            tags: tags.iter().map(|t| t.to_string()).collect(),
        }
    }
}

enum SearchElement {
    Name,
    URL,
    Group,
    Tag,
}

/// Phrase filter filters Bookmarks by specific element
/// To match, the element needs to contain the the phrase (case insensitive)
pub struct PhraseFilter {
    phrase: String,
    element: SearchElement,
}

impl Filter for PhraseFilter {
    fn matches(&self, record: &URLRecord) -> bool {
        match &self.element {
            SearchElement::Name => record.name.to_lowercase().contains(&self.phrase),
            SearchElement::URL => record.url.to_lowercase().contains(&self.phrase),
            SearchElement::Group => record.group.to_lowercase().contains(&self.phrase),
            SearchElement::Tag => tag_matches(record, &self.phrase),
        }
    }

    fn chain(self, filter: Box<dyn Filter>) -> Box<dyn Filter> {
        Box::new(FilterSet::new_combined(vec![Box::new(self), filter]))
    }
}

impl PhraseFilter {
    pub fn new_name_filter(phrase: &str) -> PhraseFilter {
        PhraseFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::Name,
        }
    }

    pub fn new_url_filter(phrase: &str) -> PhraseFilter {
        PhraseFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::URL,
        }
    }

    pub fn new_group_filter(phrase: &str) -> PhraseFilter {
        PhraseFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::Group,
        }
    }

    pub fn new_tag_filter(phrase: &str) -> PhraseFilter {
        PhraseFilter {
            phrase: phrase.to_lowercase(),
            element: SearchElement::Tag,
        }
    }
}

fn tag_matches(record: &URLRecord, word: &str) -> bool {
    for t in record.tags.keys() {
        if t.to_lowercase().contains(word) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::filters::{Filter, FilterSet, UnorderedWordSetFilter};
    use crate::types::URLRecord;

    #[test]
    fn test_unordered_word_ser_filter() {
        let test_set = vec![
            URLRecord::new(
                "http://urlAbcd.com",
                "first url",
                "default",
                vec!["pop", "with space"],
            ),
            URLRecord::new(
                "http://test123.com",
                "catchy name",
                "super group",
                vec!["pop", "with-dash"],
            ),
            URLRecord::new("http://another.com", "poppy", "group", vec![]),
        ];

        struct TestCase {
            phrase: String,
            matches: Vec<bool>,
        }

        let test_cases = vec![
            TestCase {
                phrase: "abcd url default pop".to_string(),
                matches: vec![true, false, false],
            },
            TestCase {
                phrase: "pop http com".to_string(),
                matches: vec![true, true, true],
            },
            TestCase {
                phrase: "http complicated".to_string(),
                matches: vec![false, false, false],
            },
        ];

        for test in test_cases {
            println!("Phrase: {}", test.phrase);

            let filter: UnorderedWordSetFilter = UnorderedWordSetFilter::new(test.phrase.as_str());

            for i in 0..test_set.len() {
                println!("URL: {}", &test_set[i]);
                assert_eq!(filter.matches(&test_set[i]), test.matches[i])
            }
        }
    }

    #[test]
    fn test_combined_phrase_filters() {
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

            let combined_filter: FilterSet =
                FilterSet::new_combined_for_phrase(test.phrase.as_str());

            for i in 0..test_set.len() {
                println!("URL: {}", &test_set[i]);
                assert_eq!(combined_filter.matches(&test_set[i]), test.matches[i])
            }
        }
    }
}
