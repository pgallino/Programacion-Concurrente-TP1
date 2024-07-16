#[cfg(test)]
mod tests {

    use crate::processors::*;

    /*  site1
    {"texts": ["1", "2"], "tags": ["1", "tag repetido"]}
    {"texts": ["3", "4 5 6 7"], "tags": ["2", "tag repetido"]}

        site2
    {"texts": ["8", "9"], "tags": ["3", "tag repetido"]}
    {"texts": ["10", "11 12 13 14"], "tags": ["4", "tag repetido"]}
    */

    fn assert_same_elements(expected: &[String], actual: &[String]) {
        for elem in expected {
            assert!(actual.contains(elem), "Missing element: {:?}", elem);
        }

        for elem in actual {
            assert!(expected.contains(elem), "Extra element: {:?}", elem);
        }
    }

    #[test]
    fn site_words_count_test() {
        let result_data = process_files(&list_files("/test1"));
        let words_count_site1: u32 = 7;
        let words_count_site2: u32 = 7;
        assert_eq!(
            result_data.sites.get("site1").unwrap().words,
            words_count_site1
        );
        assert_eq!(
            result_data.sites.get("site2").unwrap().words,
            words_count_site2
        );
    }

    #[test]
    fn site_questions_count_test() {
        let result_data = process_files(&list_files("/test1"));
        let questions_count_site1: u32 = 2;
        let questions_count_site2: u32 = 2;
        assert_eq!(
            result_data.sites.get("site1").unwrap().questions,
            questions_count_site1
        );
        assert_eq!(
            result_data.sites.get("site2").unwrap().questions,
            questions_count_site2
        );
    }

    #[test]
    fn tag_site_words_count_test() {
        let result_data = process_files(&list_files("/test1"));

        let expected_data = vec![
            ("site1", "1", 2),
            ("site1", "2", 5),
            ("site1", "tag repetido", 7),
            ("site2", "3", 2),
            ("site2", "4", 5),
            ("site2", "tag repetido", 7),
        ];

        for (site, tag, expected_words) in expected_data {
            let actual_words = result_data
                .sites
                .get(site)
                .and_then(|site_data| site_data.tags.get(tag).map(|tag_data| tag_data.words))
                .unwrap_or(0);

            assert_eq!(actual_words, expected_words, "Site: {}, Tag: {}", site, tag);
        }
    }

    #[test]
    fn tag_site_questions_count_test() {
        let result_data = process_files(&list_files("/test1"));

        let expected_data = vec![
            ("site1", "1", 1),
            ("site1", "2", 1),
            ("site1", "tag repetido", 2),
            ("site2", "3", 1),
            ("site2", "4", 1),
            ("site2", "tag repetido", 2),
        ];

        for (site, tag, expected_words) in expected_data {
            let actual_words = result_data
                .sites
                .get(site)
                .and_then(|site_data| site_data.tags.get(tag).map(|tag_data| tag_data.questions))
                .unwrap_or(0);

            assert_eq!(actual_words, expected_words, "Site: {}, Tag: {}", site, tag);
        }
    }

    #[test]
    fn tag_total_questions_count_test() {
        let result_data = process_files(&list_files("/test1"));

        let expected_data = vec![("1", 1), ("2", 1), ("3", 1), ("4", 1), ("tag repetido", 4)];

        for (tag, expected_questions) in expected_data {
            let actual_questions = result_data
                .tags
                .get(tag)
                .map(|tag_data| tag_data.questions)
                .unwrap_or(0);

            assert_eq!(actual_questions, expected_questions, "Tag: {}", tag);
        }
    }

    #[test]
    fn tag_total_words_count_test() {
        let result_data = process_files(&list_files("/test1"));

        let expected_data = vec![("1", 2), ("2", 5), ("3", 2), ("4", 5), ("tag repetido", 14)];

        for (tag, expected_words) in expected_data {
            let actual_words = result_data
                .tags
                .get(tag)
                .map(|tag_data| tag_data.words)
                .unwrap_or(0);

            assert_eq!(actual_words, expected_words, "Tag: {}", tag);
        }
    }

    #[test]
    fn site_chatty_tags_test() {
        let expected_site1: Vec<String> =
            vec!["1".to_string(), "2".to_string(), "tag repetido".to_string()];
        let expected_site2: Vec<String> =
            vec!["3".to_string(), "4".to_string(), "tag repetido".to_string()];
        let mut result_data = process_files(&list_files("/test1"));
        process_totals(&mut result_data);
        assert_same_elements(
            &expected_site1,
            &result_data.sites.get("site1").unwrap().chatty_tags,
        );
        assert_same_elements(
            &expected_site2,
            &result_data.sites.get("site2").unwrap().chatty_tags,
        );
    }

    #[test]
    fn total_chatty_tags_test() {
        let expected: Vec<String> = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "tag repetido".to_string(),
        ];
        let mut result_data = process_files(&list_files("/test1"));
        process_totals(&mut result_data);
        assert_same_elements(&expected, &result_data.totals.chatty_tags);
    }

    #[test]
    fn total_chatty_sites_test() {
        let expected: Vec<String> = vec!["site1".to_string(), "site2".to_string()];
        let mut result_data = process_files(&list_files("/test1"));
        process_totals(&mut result_data);
        assert_same_elements(&expected, &result_data.totals.chatty_sites);
    }
}
