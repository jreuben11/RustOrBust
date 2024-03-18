mod binary_search_tree;
mod red_black_tree;
mod heap;
mod trie;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub path: String,
    pub address: String,
}
impl IoTDevice {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> IoTDevice {
        IoTDevice {
            address: address.into(),
            numerical_id: id,
            path: path.into(),
        }
    }
}
impl PartialEq for IoTDevice {
    fn eq(&self, other: &IoTDevice) -> bool {
        self.numerical_id == other.numerical_id && self.address == other.address
    }
}



#[derive(Clone, Debug)]
pub struct MessageNotification {
    pub no_messages: u64,
    pub device: IoTDevice,
}
impl MessageNotification {
    pub fn new(device: IoTDevice, no_messages: u64) -> MessageNotification {
        MessageNotification {
            no_messages: no_messages,
            device: device,
        }
    }
}
impl PartialEq for MessageNotification {
    fn eq(&self, other: &MessageNotification) -> bool {
        self.device.eq(&other.device) && self.no_messages == other.no_messages
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::iter::FromIterator;


    fn new_device_with_id(id: u64) -> IoTDevice {
        new_device_with_id_path(id, "")
    }

    fn new_device_with_id_path(id: u64, path: impl Into<String>) -> IoTDevice {
        IoTDevice::new(id, format!("My address is {}", id), path)
    }

    fn new_notification_with_id(id: u64, no_messages: u64) -> MessageNotification {
        let dev = new_device_with_id(id);
        MessageNotification::new(dev, no_messages)
    }

    mod binary_search_tree_tests {
        use super::*;
        #[test]
        fn binary_search_tree_walk_in_order() {
            let len = 10;

            let mut tree = binary_search_tree::DeviceRegistry::new_empty();
            let mut items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

            // let mut rng = thread_rng();
            // rng.shuffle(&mut items);
            items.shuffle(&mut thread_rng());

            for item in items.iter() {
                tree.add(item.clone());
            }

            assert_eq!(tree.length, len);
            let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
            tree.walk(|n| v.borrow_mut().push(n.clone()));
            let mut items = items;
            // sort in descending order:
            items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
            assert_eq!(v.into_inner(), items)
        }

        #[test]
        fn binary_search_tree_find() {
            let mut tree = binary_search_tree::DeviceRegistry::new_empty();

            tree.add(new_device_with_id(4));
            tree.add(new_device_with_id(3));
            tree.add(new_device_with_id(2));
            tree.add(new_device_with_id(1));
            tree.add(new_device_with_id(5));
            tree.add(new_device_with_id(6));
            tree.add(new_device_with_id(7));
            assert_eq!(tree.find(100), None);
            assert_eq!(tree.find(4), Some(new_device_with_id(4)));
            assert_eq!(tree.find(3), Some(new_device_with_id(3)));
            assert_eq!(tree.find(2), Some(new_device_with_id(2)));
            assert_eq!(tree.find(1), Some(new_device_with_id(1)));
            assert_eq!(tree.find(5), Some(new_device_with_id(5)));
            assert_eq!(tree.find(6), Some(new_device_with_id(6)));
            assert_eq!(tree.find(7), Some(new_device_with_id(7)));
            assert_eq!(tree.length, 7);
        }
    }
    mod red_black_tree_tests{
        use super::*;
        
        #[test]
        fn red_black_tree_add() {
            let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();
            tree.add(new_device_with_id(1));
            tree.add(new_device_with_id(2));
            tree.add(new_device_with_id(3));
            tree.add(new_device_with_id(4));
            tree.add(new_device_with_id(5));
            tree.add(new_device_with_id(6));
            tree.add(new_device_with_id(7));
            assert_eq!(tree.length, 7);
            assert!(tree.is_a_valid_red_black_tree());
        }

        #[test]
        fn red_black_tree_walk_in_order() {
            let len = 10;

            let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();
            let mut items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();

            items.shuffle(&mut thread_rng());

            for item in items.iter() {
                tree.add(item.clone());
            }
            assert!(tree.is_a_valid_red_black_tree());
            assert_eq!(tree.length, len);
            let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
            tree.walk(|n| v.borrow_mut().push(n.clone()));
            let mut items = items;
            // sort in descending order:
            items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
            assert_eq!(v.into_inner(), items)
        }

        #[test]
        fn red_black_tree_find() {
            let mut tree = red_black_tree::BetterDeviceRegistry::new_empty();
    
            tree.add(new_device_with_id(3));
            tree.add(new_device_with_id(2));
            tree.add(new_device_with_id(1));
            tree.add(new_device_with_id(6));
            tree.add(new_device_with_id(4));
            tree.add(new_device_with_id(5));
            tree.add(new_device_with_id(7));
    
            assert!(tree.is_a_valid_red_black_tree());
            assert_eq!(tree.length, 7);
    
            assert_eq!(tree.find(100), None);
            assert_eq!(tree.find(4), Some(new_device_with_id(4)));
            assert_eq!(tree.find(3), Some(new_device_with_id(3)));
            assert_eq!(tree.find(2), Some(new_device_with_id(2)));
            assert_eq!(tree.find(1), Some(new_device_with_id(1)));
            assert_eq!(tree.find(5), Some(new_device_with_id(5)));
            assert_eq!(tree.find(6), Some(new_device_with_id(6)));
            assert_eq!(tree.find(7), Some(new_device_with_id(7)));
        }

    }


    mod heap_tests {
        use super::*;
        #[test]
        fn binary_heap_add() {
            let mut heap = heap::MessageChecker::new_empty();

            heap.add(new_notification_with_id(1, 100));
            heap.add(new_notification_with_id(2, 200));
            heap.add(new_notification_with_id(3, 500));
            heap.add(new_notification_with_id(4, 40));
            assert_eq!(heap.length, 4);
        }

        #[test]
        fn binary_heap_pop() {
            let mut heap = heap::MessageChecker::new_empty();

            let a = new_notification_with_id(1, 40);
            let b = new_notification_with_id(2, 300);
            let c = new_notification_with_id(3, 50);
            let d = new_notification_with_id(4, 500);

            heap.add(a.clone());
            heap.add(b.clone());
            heap.add(c.clone());
            heap.add(d.clone());

            assert_eq!(heap.length, 4);

            assert_eq!(heap.pop(), Some(d));
            assert_eq!(heap.pop(), Some(b));
            assert_eq!(heap.pop(), Some(c));
            assert_eq!(heap.pop(), Some(a));
        }

    }

    mod trie_tests {
        use super::*;

        #[test]
        fn trie_add() {
            let mut trie = trie::BestDeviceRegistry::new_empty();
            let len = 10;

            let mut rng = thread_rng();

            for i in 0..len {
                trie.add(new_device_with_id_path(
                    i,
                    format!("factory{}/machineA/{}", rng.gen_range(0..len), i),
                ));
            }

            assert_eq!(trie.length, len);
        }

        #[test]
        fn trie_walk_in_order() {
            let mut trie = trie::BestDeviceRegistry::new_empty();
            let len = 10;

            let mut rng = thread_rng();
            let items: Vec<IoTDevice> = (0..len)
                .map(|i| {
                    new_device_with_id_path(
                        i,
                        format!("factory{}/machineA/{}", rng.gen_range(0..len), i),
                    )
                })
                .collect();

            for item in items.iter() {
                trie.add(item.clone());
            }
            assert_eq!(trie.length, len);
            let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
            trie.walk(|n| v.borrow_mut().push(n.clone()));
            let mut items = items;
            // sort in descending order:
            items.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
            let mut actual = v.into_inner();
            actual.sort_by(|a, b| b.numerical_id.cmp(&a.numerical_id));
            assert_eq!(actual, items)
        }

        #[test]
        fn trie_find() {
            let mut trie = trie::BestDeviceRegistry::new_empty();
            let len = 10;

            let mut rng = thread_rng();
            let mut paths = vec![];
            for i in 0..len {
                let s = format!("factory{}/machineA/{}", rng.gen_range(0..len), i);
                trie.add(new_device_with_id_path(i, s.clone()));
                paths.push(s);
            }

            assert_eq!(trie.length, len);
            assert_eq!(trie.find("100"), None);
        }
    }
    

}
