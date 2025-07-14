// This function returns how much icecream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no icecream is left (value 0). Return `None` if
// `hour_of_day` is higher than 23.
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    // TODO: Complete the function body.
    if hour_of_day < 22 {
        Some(5)
    } else if hour_of_day > 23 {
        None
    } else {
        Some(0)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // Option?
        let icecreams: u16 = maybe_icecream(12).unwrap();

        assert_eq!(icecreams, 5); // Don't change this line.
    }

    // This test fails because the implementation of maybe_icecream is incorrect for hour 23.
    // The function currently returns None for any hour greater than 22, but the test expects
    // Some(0) for hour 23. According to the test, at 22:00 and 23:00, there should be 0 icecreams left,
    // but the function only returns Some(0) at exactly 22, and None for 23 and above.
    //
    // To fix the test, you would need to either change the function to return Some(0) for both 22 and 23,
    // or change the test to expect None for hour 23.
    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0)); // This line fails with the current implementation.
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
