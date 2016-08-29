extern crate hashtable_test;
use hashtable_test::HashTable;

#[test]
fn test_i32key() {
    let mut table = HashTable::new();

    table.insert(123, "Asdf");
    table.insert(456, "Ghjk");

    for i in 1..50 {
        table.insert(i, "test");
    }

    assert!(table.get(&123) == Some(&"Asdf"));
    assert!(table.get(&456) == Some(&"Ghjk"));
    assert!(table.get(&0) == None);
    assert!(table.get(&122) == None);

    for i in 1..50 {
        assert!(table.get(&i) == Some(&"test"));
    }
    assert!(table.get(&50) == None);
}

#[test]
fn test_strkey() {
    let mut table = HashTable::new();

    table.insert("Asdf", 123);
    table.insert("Ghjk", 456);

    assert!(table.get(&"Asdf") == Some(&123));
    assert!(table.get(&"Ghjk") == Some(&456));
    assert!(table.get(&"") == None);
    assert!(table.get(&"Asd") == None);
}

