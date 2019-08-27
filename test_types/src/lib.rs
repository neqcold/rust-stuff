mod types;

#[cfg(test)]
mod tests {
    use crate::types::*;

    #[test]
    fn test_volume() {
        assert_eq!(MyBox{ width: 5, length: 4, height: 3}.volume(), 5 * 4 * 3);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = MyBox{ width: 10, length: 5, height: 20 };
        let smaller = MyBox{ width: 8, length :2, height: 5 };
        assert!(larger.can_hold(&smaller));

        let larger = MyBox{ width: 5, length: 20, height: 10 };
        let smaller = MyBox{ width: 15, length: 4, height: 8 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn create_even_number() {
        let n = EvenNumber::new(4);
        assert_eq!(n.value(), 4);
    }

    #[test]
    #[should_panic]
    fn cannot_put_noteven_in_even() {
        let _ = EvenNumber::new(3);
    }

}
