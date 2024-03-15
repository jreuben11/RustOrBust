mod singly_linked_list;
mod doubly_linked_list;

#[cfg(test)]
mod tests {
    use crate::*;
    

    #[test]
    fn transaction_log_append() {
        let mut transaction_log = singly_linked_list::TransactionLog::new_empty();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(transaction_log.pop(), None);
    }

    #[test]
    fn transaction_log_pop() {
        let mut list = singly_linked_list::TransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(list.pop(), None);
    }


    #[test]
    fn better_transaction_log_append() {
        let mut transaction_log = doubly_linked_list::BetterTransactionLog::new_empty();
        assert_eq!(transaction_log.length, 0);
        transaction_log.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        transaction_log.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        assert_eq!(transaction_log.length, 3);
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            transaction_log.pop(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(transaction_log.pop(), None);
    }


    #[test]
    fn better_transaction_log_pop() {
        let mut list = doubly_linked_list::BetterTransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(list.pop(), None);
    }


    #[test]
    fn better_transaction_log_iterator() {
        let mut list = doubly_linked_list::BetterTransactionLog::new_empty();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        list.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        let mut iter = list.clone().into_iter();
        assert_eq!(
            iter.next(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            iter.next(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            iter.next(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );

        let mut iter = list.clone().back_iter();
        assert_eq!(
            iter.next_back(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(
            iter.next_back(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            iter.next_back(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
    }

}