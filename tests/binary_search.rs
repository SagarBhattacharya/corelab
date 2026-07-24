use corelab::binary_search::binary_search;
use rand::random_range;

#[test]
fn test_binary_search_cases() {
    let tests = [
        ("empty array", &[][..], 2, None),
        ("one element array", &[5], 5, Some(0)),
        ("found first element", &[1, 2, 3, 4], 1, Some(0)),
        ("found last element", &[1, 2, 3, 4], 4, Some(3)),
        ("found middle element", &[1, 2, 3, 4, 5], 3, Some(2)),
        ("element not found", &[1, 2, 3, 4, 5], 6, None),
        (
            "negative element",
            &[-3, -2, -1, 0, 1, 2, 3, 4],
            -2,
            Some(1),
        ),
    ];

    for (name, arr, target, expected) in tests {
        assert_eq!(binary_search(arr, &target), expected, "test failed: {name}");
    }
}

#[test]
fn test_duplicate_values() {
    let arr = [1, 2, 3, 4, 4, 5, 6, 7];
    let idx = binary_search(&arr, &4).unwrap();
    assert_eq!(arr[idx], 4);
}

#[test]
fn differential_test_stdlib() {
    for _ in 0..10_000 {
        let size = random_range(0..100);

        let mut arr: Vec<i32> = (0..size).map(|_| random_range(-1000..1000)).collect();

        arr.sort();

        let target = random_range(-1000..1000);

        let mine = binary_search(&arr, &target);
        let std = arr.binary_search(&target).ok();

        match (mine, std) {
            (Some(i), Some(j)) => {
                assert_eq!(arr[i], target);
                assert_eq!(arr[j], target);
            }
            (None, None) => {}
            _ => panic!("Results differ!"),
        }
    }
}
