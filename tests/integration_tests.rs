use droprate::RateTable;

#[test]
fn empty_table() {
    let table = RateTable::new();
    assert_eq!(table.count(), 0);
}