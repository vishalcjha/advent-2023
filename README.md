## Advent of code 2023.

Happy Coding !!!
25 things from 25 days of challange.

### Learning from each day

#### Day 3
Lots of coding challange revolves around finding neighbours, given matrix as input.
Some examples are finding islands, bfs walk etc.
Defination of neighbours usually comes in two flavor
a. Element up, down, left, right from current element.
b. All element in a but also 4 more diagonal elements w.r.t. current element.
This results in follwing looking code - 
```
    let (cr, cc) = get_current_row_col();
    for rx in [0, 1, -1] {
        for cx in [0, 1, -1] {
            if rx == cx && rx == 0 {
                continue;
            }
            /* uncomment me if condition a
            if rx.abs() == cx.abs() && rx.abs() == 1 {
                continue;
            }*/
            let neighbour_row = cr + rx;
            let neighbour_y = cc + cx;
            if !is_valid(neighbour_row, neighbour_y) {
                continue;
            }
            // process neighbhour cell.
        }
    }
```
#### Day 4 
`filter_map` -> Iterator have adapter filter_map which chains map and filter together.
Say we have String with splitted by space. Some of splitted string can be conveted to numbers aka u32 and some not.
Without filter_map we can do following 
```
let parsed_numbers = line.split_ascii_whitespace()
            .map(|num| num.parse::<u32>().ok())
            .filter(|it| it.is_some())
            // this is required as map does not knows that all None has been removed. Ugh
            .map(|it| it.unwrap())
            .collect::<Vec<_>>();
```
We have lots of adapter and it is also not elegant as we are forced to use `unwrap`.
But map and filter can be combined together to get same effect as below
```
let parsed_numbers = line.split_ascii_whitespace()
                          .filter_map(|num| num.parse::<u32>().ok())
                          .collect::<Vec<_>>();
```

#### Day 6
A classic binary search problem. In simplified terms you have given a sorted list of numbers. You have a single possible set of indices which matches creteria. How would you find it. I will define how to find left most indices.
```
    pub(self) fn find_left_end(&self, left: u128, right: u128) -> Option<u128> {
        let mut smallest_winning_left = None;
        if left <= right {
            let mid = left + (right - left) / 2;

            if self.will_win_in_time(mid) {
                // if mid is solution, check indices before mid for answer.
                smallest_winning_left = smallest_winning_left.or(Some(mid));
                if let Some(other_possible) = self.find_left_end(left, mid - 1) {
                    smallest_winning_left =
                        smallest_winning_left.and_then(|current| Some(current.min(other_possible)));
                }
            } else {
                // if mid is not solution, first try range before mid if not try range after mid.
                if let Some(other_possible) = self.find_left_end(left, mid - 1) {
                    smallest_winning_left =
                        smallest_winning_left.and_then(|current| Some(current.min(other_possible)));
                } else {
                    smallest_winning_left = self.find_left_end(mid + 1, right)
                }
            }
        }
        smallest_winning_left
    }
```
Right end can be found in similar way.

More instrestingly variance comes into play with lifetime and reference. Will need to find time to describe the problem.