#[cfg(test)]
mod tests {
    use exitfailure::ExitFailure;
    use crate::{HistoryItems, sort};

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn sort_sorts_alphabetically() -> Result<(), ExitFailure> {
        let history_items = HistoryItems::new();
        let mut items = vec_of_strings!["Insync", "Firefox", "Spotify"];
        let sorted_items = vec_of_strings!["Firefox", "Insync", "Spotify"];

        sort(&history_items, &mut items)?;

        assert_eq!(items, sorted_items);

        Ok(())
    }

    #[test]
    fn sort_sorts_historical_items_first() -> Result<(), ExitFailure> {
        let mut history_items = HistoryItems::new();

        history_items.insert("Insync".to_string(), 1);
        history_items.insert("Spotify".to_string(), 2);

        let mut items = vec_of_strings!["Insync", "Firefox", "Spotify"];
        let sorted_items = vec_of_strings!["Spotify", "Insync", "Firefox"];

        sort(&history_items, &mut items)?;

        assert_eq!(items, sorted_items);

        Ok(())
    }

    #[test]
    fn sort_sorts_case_insensitive() -> Result<(), ExitFailure> {
        let history_items = HistoryItems::new();
        let mut items = vec_of_strings!["insync", "Firefox", "Spotify"];
        let sorted_items = vec_of_strings!["Firefox", "insync", "Spotify"];

        sort(&history_items, &mut items)?;

        assert_eq!(items, sorted_items);

        Ok(())
    }
}
