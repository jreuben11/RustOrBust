mod binary_search_tree;

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
