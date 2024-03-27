#[cfg(test)]
mod dll_tests {
    use dll::dll::List;

    #[test]
    fn test_push_back()
    {
        // ARRANGE
        let mut list = List::<String>::new();

        // ACT
        list.push_back("first".to_owned());
        list.push_back("second".to_owned());
        list.push_back("third".to_owned());

        // ASSERT
        assert_eq!(list.size(), 3);
        // CHECK DATA
        assert_eq!(list.peek_back().unwrap(), "third");
        assert_eq!(list.peek_front().unwrap(), "first");

    }

    #[test]
    fn test_push_front()
    {
        // ARRANGE
        let mut list = List::<String>::new();

        // ACT
        list.push_front("first".to_owned());
        list.push_front("second".to_owned());
        list.push_front("third".to_owned());

        // ASSERT
        assert_eq!(list.size(), 3);
        // CHECK DATA
        assert_eq!(list.peek_back().unwrap(), "first");
        assert_eq!(list.peek_front().unwrap(), "third");
    }

    #[test]
    fn test_pop_back()
    {
        // ASSERT
        let mut list = List::new();

        // ACT
        list.push_back("first".to_owned());
        list.push_back("second".to_owned());
        list.push_back("third".to_owned());

        // ASSERT
        assert_eq!(list.pop_back().unwrap(), "third");
        assert_eq!(list.pop_back().unwrap(), "second");
        assert_eq!(list.pop_back().unwrap(), "first");
    }

    #[test]
    fn test_pop_front()
    {
        // ASSERT
        let mut list = List::new();

        // ACT
        list.push_back("first".to_owned());
        list.push_back("second".to_owned());
        list.push_back("third".to_owned());

        // ASSERT
        assert_eq!(list.pop_front().unwrap(), "first");
        assert_eq!(list.pop_front().unwrap(), "second");
        assert_eq!(list.pop_front().unwrap(), "third");
    }
}