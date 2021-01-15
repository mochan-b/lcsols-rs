pub struct Solution();

impl Solution {
    pub fn find_heater(house: i32, heaters: &Vec<i32>) -> usize {
        let mut left = 0;
        let mut right = heaters.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            if heaters[mid as usize] == house {
                return mid;
            } else if heaters[mid as usize] < house {
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
        }

        left
    }

    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses_sorted = houses;
        houses_sorted.sort_unstable();
        let mut heaters_sorted = heaters;
        heaters_sorted.sort_unstable();

        let mut radius = 0;

        for house in houses_sorted {
            let heater_index = Self::find_heater(house, &heaters_sorted);

            let left_dist_calc = || house - heaters_sorted[heater_index - 1];
            let right_dist_calc = || heaters_sorted[heater_index] - house;

            radius = radius.max(
                if heater_index == 0 {
                    right_dist_calc()
                } else if heater_index == heaters_sorted.len() {
                    left_dist_calc()
                } else {
                    left_dist_calc().min(right_dist_calc())
                }
            );
        }

        radius
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let houses = vec![1, 2, 3];
        let heaters = vec![2];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(1, result);
    }

    #[test]
    fn test2() {
        let houses = vec![1, 3, 2];
        let heaters = vec![2];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(1, result);
    }

    #[test]
    fn test3() {
        let houses = vec![1, 2, 3, 4];
        let heaters = vec![1, 4];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(1, result);
    }

    #[test]
    fn test4() {
        let houses = vec![1, 5];
        let heaters = vec![2];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(3, result);
    }

    #[test]
    fn test5() {
        let houses = vec![1, 5];
        let heaters = vec![10];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(9, result);
    }

    #[test]
    fn test6() {
        let houses = vec![282475249, 622650073, 984943658, 144108930, 470211272, 101027544, 457850878, 458777923];
        let heaters = vec![823564440, 115438165, 784484492, 74243042, 114807987, 137522503, 441282327, 16531729, 823378840, 143542612];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(161834419, result);
    }

    #[test]
    fn test7() {
        let houses = vec![474833169, 264817709, 998097157, 817129560];
        let heaters = vec![197493099, 404280278, 893351816, 505795335];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(104745341, result);
    }

    #[test]
    fn test8() {
        let houses = vec![1];
        let heaters = vec![100];

        let result = Solution::find_radius(houses, heaters);

        assert_eq!(99, result);
    }
}
