pub fn chop_iterative(search_target: i64, search_space: Vec<i64>) -> i64 {
    let mut current_search_space = search_space;
    let mut offset = 0 as i64;

    while current_search_space.len() > 0 {
        let midpoint = (current_search_space.len() / 2) as i64;

        if current_search_space.len() <= 2 {
            match current_search_space
                .iter()
                .position(|&r| r == search_target)
            {
                Some(index) => return index as i64 + offset,
                None => return -1,
            }
        }

        if current_search_space[midpoint as usize] == search_target {
            return midpoint + offset;
        } else if current_search_space[midpoint as usize] > search_target {
            current_search_space = current_search_space[..midpoint as usize].to_vec();
        } else if current_search_space[midpoint as usize] < search_target {
            current_search_space = current_search_space[midpoint as usize..].to_vec();
            offset += midpoint
        }
    }

    -1
}

pub fn chop_stack(search_target: i64, search_space: Vec<i64>) -> i64 {
    let midpoint = (search_space.len() / 2) as i64;

    if search_space.len() <= 2 {
        match search_space.iter().position(|&r| r == search_target) {
            Some(index) => return index as i64,
            None => return -1,
        }
    }

    if search_space[midpoint as usize] == search_target {
        return midpoint;
    } else if search_space[midpoint as usize] > search_target {
        return chop_stack(search_target, search_space[..midpoint as usize].to_vec());
    } else if search_space[midpoint as usize] < search_target {
        let search_result = chop_stack(search_target, search_space[midpoint as usize..].to_vec());

        match search_result {
            -1 => return -1,
            _ => return midpoint + search_result,
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use chop_iterative;
    use chop_stack;

    #[test]
    fn case_iterative_01() {
        assert_eq!(-1, chop_iterative(3, vec![]))
    }

    #[test]
    fn case_iterative_02() {
        assert_eq!(-1, chop_iterative(3, vec![]))
    }

    #[test]
    fn case_iterative_03() {
        assert_eq!(-1, chop_iterative(3, vec![1]))
    }

    #[test]
    fn case_iterative_04() {
        assert_eq!(0, chop_iterative(1, vec![1]))
    }

    #[test]
    fn case_iterative_05() {
        assert_eq!(0, chop_iterative(1, vec![1, 3, 5]))
    }

    #[test]
    fn case_iterative_06() {
        assert_eq!(1, chop_iterative(3, vec![1, 3, 5]))
    }

    #[test]
    fn case_iterative_07() {
        assert_eq!(2, chop_iterative(5, vec![1, 3, 5]))
    }

    #[test]
    fn case_iterative_08() {
        assert_eq!(-1, chop_iterative(0, vec![1, 3, 5]))
    }

    #[test]
    fn case_iterative_09() {
        assert_eq!(-1, chop_iterative(2, vec![1, 3, 5]))
    }

    #[test]
    fn case_iterative_10() {
        assert_eq!(-1, chop_iterative(4, vec![1, 3, 5]))
    }

    #[test]
    fn case_iterative_11() {
        assert_eq!(-1, chop_iterative(6, vec![1, 3, 5]))
    }

    #[test]
    fn case_iterative_12() {
        assert_eq!(0, chop_iterative(1, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_13() {
        assert_eq!(1, chop_iterative(3, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_14() {
        assert_eq!(2, chop_iterative(5, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_15() {
        assert_eq!(3, chop_iterative(7, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_16() {
        assert_eq!(-1, chop_iterative(0, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_17() {
        assert_eq!(-1, chop_iterative(2, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_18() {
        assert_eq!(-1, chop_iterative(4, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_19() {
        assert_eq!(-1, chop_iterative(6, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_iterative_20() {
        assert_eq!(-1, chop_iterative(8, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_01() {
        assert_eq!(-1, chop_stack(3, vec![]))
    }

    #[test]
    fn case_stack_02() {
        assert_eq!(-1, chop_stack(3, vec![]))
    }

    #[test]
    fn case_stack_03() {
        assert_eq!(-1, chop_stack(3, vec![1]))
    }

    #[test]
    fn case_stack_04() {
        assert_eq!(0, chop_stack(1, vec![1]))
    }

    #[test]
    fn case_stack_05() {
        assert_eq!(0, chop_stack(1, vec![1, 3, 5]))
    }

    #[test]
    fn case_stack_06() {
        assert_eq!(1, chop_stack(3, vec![1, 3, 5]))
    }

    #[test]
    fn case_stack_07() {
        assert_eq!(2, chop_stack(5, vec![1, 3, 5]))
    }

    #[test]
    fn case_stack_08() {
        assert_eq!(-1, chop_stack(0, vec![1, 3, 5]))
    }

    #[test]
    fn case_stack_09() {
        assert_eq!(-1, chop_stack(2, vec![1, 3, 5]))
    }

    #[test]
    fn case_stack_10() {
        assert_eq!(-1, chop_stack(4, vec![1, 3, 5]))
    }

    #[test]
    fn case_stack_11() {
        assert_eq!(-1, chop_stack(6, vec![1, 3, 5]))
    }

    #[test]
    fn case_stack_12() {
        assert_eq!(0, chop_stack(1, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_13() {
        assert_eq!(1, chop_stack(3, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_14() {
        assert_eq!(2, chop_stack(5, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_15() {
        assert_eq!(3, chop_stack(7, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_16() {
        assert_eq!(-1, chop_stack(0, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_17() {
        assert_eq!(-1, chop_stack(2, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_18() {
        assert_eq!(-1, chop_stack(4, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_19() {
        assert_eq!(-1, chop_stack(6, vec![1, 3, 5, 7]))
    }

    #[test]
    fn case_stack_20() {
        assert_eq!(-1, chop_stack(8, vec![1, 3, 5, 7]))
    }
}
