<div align="center">

# 🦀 100 Rust Problems - Solutions


**My personal solutions for the [100 Rust Interview Problems](https://github.com/aarambh-darshan/100-rust-problems) by [Darshan Vichhi](https://github.com/aarambh-darshan)**


[📚 Problems](#-problem-categories) • [🚀Running Solutions](#-running-solutions) • [� Progress](#-progress-tracker) • [🤝 Contributing](#-contributing)

</div>

---

## 📁 Project Structure

```
rust-100-problems/
├── 📄 Cargo.toml
├── 📖 README.md
└── 📂 src/
    ├── main.rs
    └── solutions/
        ├── mod.rs                    # Module declarations
        │
        ├── 🟢 Beginner (001-035)
        │   ├── p001_two_sum.rs
        │   ├── p002_reverse_string.rs
        │   └── ...
        │
        ├── 🟡 Intermediate (036-070)
        │   ├── p036_implement_stack.rs
        │   ├── p047_two_sum_hashmap.rs
        │   └── ...
        │
        └── 🔴 Advanced (071-100)
            ├── p071_house_robber.rs
            ├── p092_lru_cache.rs
            └── ...
```

---

## 🚀 Running Solutions

```bash
# Run all tests
cargo test

# Run tests for a specific problem
cargo test p001                    # Two Sum
cargo test p042                    # Linked List Cycle
cargo test p092                    # LRU Cache

# Run with output
cargo test p001 -- --nocapture
```

---

## � Problem Categories

### 🟢 Beginner (1-35)

<details>
<summary><b>Basic Syntax & Math (1-15)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 001 | [Two Sum](src/solutions/p001_two_sum.rs) | 🟢 | Arrays, Hash Map |
| 002 | [Reverse String](src/solutions/p002_reverse_string.rs) | 🟢 | Strings, Two Pointers |
| 003 | [Palindrome Number](src/solutions/p003_palindrome_number.rs) | 🟢 | Math |
| 004 | [FizzBuzz](src/solutions/p004_fizzbuzz.rs) | 🟢 | Loops, Conditionals |
| 005 | [Fibonacci Number](src/solutions/p005_fibonacci_number.rs) | 🟢 | Recursion, DP |
| 006 | [Factorial](src/solutions/p006_factorial.rs) | 🟢 | Recursion |
| 007 | [Count Digits](src/solutions/p007_count_digits.rs) | 🟢 | Math |
| 008 | [Sum of Array](src/solutions/p008_sum_of_array.rs) | 🟢 | Arrays |
| 009 | [Find Maximum](src/solutions/p009_find_maximum.rs) | 🟢 | Arrays |
| 010 | [Find Minimum](src/solutions/p010_find_minimum.rs) | 🟢 | Arrays |
| 011 | [Even or Odd](src/solutions/p011_even_or_odd.rs) | 🟢 | Math |
| 012 | [Prime Number Check](src/solutions/p012_prime_number_check.rs) | 🟢 | Math |
| 013 | [Leap Year](src/solutions/p013_leap_year.rs) | 🟢 | Math |
| 014 | [Celsius to Fahrenheit](src/solutions/p014_celsius_to_fahrenheit.rs) | 🟢 | Math |
| 015 | [Swap Two Numbers](src/solutions/p015_swap_two_numbers.rs) | 🟢 | Variables |

</details>

<details>
<summary><b>Arrays & Strings (16-35)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 016 | [Reverse Integer](src/solutions/p016_reverse_integer.rs) | 🟢 | Math |
| 017 | [Count Vowels](src/solutions/p017_count_vowels.rs) | 🟢 | Strings |
| 018 | [Remove Duplicates](src/solutions/p018_remove_duplicates_from_sorted_array.rs) | 🟢 | Two Pointers |
| 019 | [Merge Sorted Arrays](src/solutions/p019_merge_two_sorted_arrays.rs) | 🟢 | Arrays |
| 020 | [Valid Parentheses](src/solutions/p020_valid_parentheses.rs) | 🟢 | Stack |
| 021 | [Plus One](src/solutions/p021_plus_one.rs) | 🟢 | Arrays |
| 022 | [Sqrt(x)](src/solutions/p022_sqrt_x.rs) | 🟢 | Binary Search |
| 023 | [Climbing Stairs](src/solutions/p023_climbing_stairs.rs) | 🟢 | DP |
| 024 | [Remove Element](src/solutions/p024_remove_element.rs) | 🟢 | Two Pointers |
| 025 | [Search Insert Position](src/solutions/p025_search_insert_position.rs) | 🟢 | Binary Search |
| 026 | [Length of Last Word](src/solutions/p026_length_of_last_word.rs) | 🟢 | Strings |
| 027 | [Add Binary](src/solutions/p027_add_binary.rs) | 🟢 | Strings, Math |
| 028 | [Single Number](src/solutions/p028_single_number.rs) | 🟢 | Bit Manipulation |
| 029 | [Best Time to Buy Stock](src/solutions/p029_best_time_to_buy_and_sell_stock.rs) | 🟢 | Arrays, DP |
| 030 | [Pascal's Triangle](src/solutions/p030_pascals_triangle.rs) | 🟢 | Arrays |
| 031 | [Valid Anagram](src/solutions/p031_valid_anagram.rs) | 🟢 | Hash Map |
| 032 | [First Unique Character](src/solutions/p032_first_unique_character.rs) | 🟢 | Hash Map |
| 033 | [Intersection of Arrays](src/solutions/p033_intersection_of_two_arrays.rs) | 🟢 | Hash Set |
| 034 | [Move Zeroes](src/solutions/p034_move_zeroes.rs) | 🟢 | Two Pointers |
| 035 | [Power of Two](src/solutions/p035_power_of_two.rs) | 🟢 | Bit Manipulation |

</details>

### 🟡 Intermediate (36-70)

<details>
<summary><b>Data Structures (36-45)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 036 | [Implement Stack](src/solutions/p036_implement_stack.rs) | 🟡 | Stack |
| 037 | [Implement Queue](src/solutions/p037_implement_queue.rs) | 🟡 | Queue |
| 038 | [Evaluate RPN](src/solutions/p038_evaluate_reverse_polish_notation.rs) | 🟡 | Stack |
| 039 | [Min Stack](src/solutions/p039_min_stack.rs) | 🟡 | Stack, Design |
| 040 | [Reverse Linked List](src/solutions/p040_reverse_linked_list.rs) | 🟡 | Linked List |
| 041 | [Merge Sorted Lists](src/solutions/p041_merge_two_sorted_lists.rs) | 🟡 | Linked List |
| 042 | [Linked List Cycle](src/solutions/p042_linked_list_cycle.rs) | 🟡 | Linked List, Floyd's |
| 043 | [Middle of Linked List](src/solutions/p043_middle_of_linked_list.rs) | 🟡 | Linked List |
| 044 | [Remove Nth From End](src/solutions/p044_remove_nth_from_end.rs) | 🟡 | Linked List |
| 045 | [Palindrome Linked List](src/solutions/p045_palindrome_linked_list.rs) | 🟡 | Linked List |

</details>

<details>
<summary><b>HashMaps & HashSets (46-50)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 046 | [Contains Duplicate](src/solutions/p046_contains_duplicate.rs) | 🟡 | Hash Set |
| 047 | [Two Sum HashMap](src/solutions/p047_two_sum_hashmap.rs) | 🟡 | Hash Map |
| 048 | [Group Anagrams](src/solutions/p048_group_anagrams.rs) | 🟡 | Hash Map, Strings |
| 049 | [Top K Frequent](src/solutions/p049_top_k_frequent_elements.rs) | 🟡 | Hash Map, Heap |
| 050 | [Longest Consecutive](src/solutions/p050_longest_consecutive_sequence.rs) | 🟡 | Hash Set |

</details>

<details>
<summary><b>Binary Search & Sorting (51-56)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 051 | [Binary Search](src/solutions/p051_binary_search.rs) | 🟡 | Binary Search |
| 052 | [Search Rotated Array](src/solutions/p052_search_in_rotated_sorted_array.rs) | 🟡 | Binary Search |
| 053 | [Find Peak Element](src/solutions/p053_find_peak_element.rs) | 🟡 | Binary Search |
| 054 | [First Bad Version](src/solutions/p054_first_bad_version.rs) | 🟡 | Binary Search |
| 055 | [Search 2D Matrix](src/solutions/p055_search_2d_matrix.rs) | 🟡 | Binary Search |
| 056 | [Sort Colors](src/solutions/p056_sort_colors.rs) | 🟡 | Dutch Flag |

</details>

<details>
<summary><b>Arrays & Intervals (57-67)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 057 | [Merge Intervals](src/solutions/p057_merge_intervals.rs) | 🟡 | Intervals, Sorting |
| 058 | [Insert Interval](src/solutions/p058_insert_interval.rs) | 🟡 | Intervals |
| 059 | [Meeting Rooms](src/solutions/p059_meeting_rooms.rs) | 🟡 | Intervals |
| 060 | [3Sum](src/solutions/p060_3sum.rs) | 🟡 | Two Pointers |
| 061 | [Container With Water](src/solutions/p061_container_with_most_water.rs) | 🟡 | Two Pointers |
| 062 | [Product Except Self](src/solutions/p062_product_of_array_except_self.rs) | 🟡 | Arrays |
| 063 | [Maximum Subarray](src/solutions/p063_maximum_subarray.rs) | 🟡 | Kadane's Algorithm |
| 064 | [Spiral Matrix](src/solutions/p064_spiral_matrix.rs) | 🟡 | Matrix |
| 065 | [Rotate Image](src/solutions/p065_rotate_image.rs) | 🟡 | Matrix |
| 066 | [Set Matrix Zeroes](src/solutions/p066_set_matrix_zeroes.rs) | 🟡 | Matrix |
| 067 | [Valid Sudoku](src/solutions/p067_valid_sudoku.rs) | 🟡 | Hash Set, Matrix |

</details>

<details>
<summary><b>Backtracking (68-70)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 068 | [Subsets](src/solutions/p068_subsets.rs) | 🟡 | Backtracking |
| 069 | [Permutations](src/solutions/p069_permutations.rs) | 🟡 | Backtracking |
| 070 | [Combination Sum](src/solutions/p070_combination_sum.rs) | 🟡 | Backtracking |

</details>

### 🔴 Advanced (71-100)

<details>
<summary><b>Dynamic Programming (71-80)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 071 | [House Robber](src/solutions/p071_house_robber.rs) | 🔴 | DP |
| 072 | [Coin Change](src/solutions/p072_coin_change.rs) | 🔴 | DP |
| 073 | [Longest Increasing Subsequence](src/solutions/p073_longest_increasing_subsequence.rs) | 🔴 | DP, Binary Search |
| 074 | [Unique Paths](src/solutions/p074_unique_paths.rs) | 🔴 | DP |
| 075 | [Jump Game](src/solutions/p075_jump_game.rs) | 🔴 | Greedy, DP |
| 076 | [Word Break](src/solutions/p076_word_break.rs) | 🔴 | DP, Trie |
| 077 | [Decode Ways](src/solutions/p077_decode_ways.rs) | 🔴 | DP |
| 078 | [Longest Common Subsequence](src/solutions/p078_longest_common_subsequence.rs) | 🔴 | DP |
| 079 | [Edit Distance](src/solutions/p079_edit_distance.rs) | 🔴 | DP |
| 080 | [Maximal Square](src/solutions/p080_maximal_square.rs) | 🔴 | DP, Matrix |

</details>

<details>
<summary><b>Graph Algorithms (81-90)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 081 | [Number of Islands](src/solutions/p081_number_of_islands.rs) | 🔴 | DFS, BFS |
| 082 | [Clone Graph](src/solutions/p082_clone_graph.rs) | 🔴 | DFS, Hash Map |
| 083 | [Course Schedule](src/solutions/p083_course_schedule.rs) | 🔴 | Topological Sort |
| 084 | [Pacific Atlantic](src/solutions/p084_pacific_atlantic_water_flow.rs) | 🔴 | DFS, BFS |
| 085 | [Word Ladder](src/solutions/p085_word_ladder.rs) | 🔴 | BFS |
| 086 | [Alien Dictionary](src/solutions/p086_alien_dictionary.rs) | 🔴 | Topological Sort |
| 087 | [Graph Valid Tree](src/solutions/p087_graph_valid_tree.rs) | 🔴 | Union Find |
| 088 | [Shortest Path Binary Matrix](src/solutions/p088_shortest_path_binary_matrix.rs) | 🔴 | BFS |
| 089 | [Network Delay Time](src/solutions/p089_network_delay_time.rs) | 🔴 | Dijkstra's |
| 090 | [Minimum Spanning Tree](src/solutions/p090_minimum_spanning_tree.rs) | 🔴 | Kruskal's/Prim's |

</details>

<details>
<summary><b>System Design & Advanced (91-100)</b></summary>

| # | Problem | Difficulty | Topics |
|:-:|---------|:----------:|--------|
| 091 | [Implement Trie](src/solutions/p091_implement_trie.rs) | 🔴 | Trie |
| 092 | [LRU Cache](src/solutions/p092_lru_cache.rs) | 🔴 | Hash Map, Linked List |
| 093 | [Serialize/Deserialize Tree](src/solutions/p093_serialize_deserialize_tree.rs) | 🔴 | Trees, DFS |
| 094 | [Design Twitter](src/solutions/p094_design_twitter.rs) | 🔴 | OOP, Heap |
| 095 | [Find Median Stream](src/solutions/p095_find_median_from_data_stream.rs) | 🔴 | Heap |
| 096 | [Sliding Window Maximum](src/solutions/p096_sliding_window_maximum.rs) | 🔴 | Deque |
| 097 | [Trapping Rain Water](src/solutions/p097_trapping_rain_water.rs) | 🔴 | Two Pointers, Stack |
| 098 | [Merge K Sorted Lists](src/solutions/p098_merge_k_sorted_lists.rs) | 🔴 | Heap, Linked List |
| 099 | [Regular Expression](src/solutions/p099_regular_expression_matching.rs) | 🔴 | DP, Recursion |
| 100 | [N-Queens](src/solutions/p100_n_queens.rs) | 🔴 | Backtracking |

</details>

---

## 📊 Progress Tracker

📋 [PROGRESS.md](PROGRESS.md)

| Difficulty | Total | Completed | Progress |
|:----------:|:-----:|:---------:|:--------:|
| 🟢 Beginner | 35 | 0 | ![](https://geps.dev/progress/0) |
| 🟡 Intermediate | 35 | 0 | ![](https://geps.dev/progress/0) |
| 🔴 Advanced | 30 | 0 | ![](https://geps.dev/progress/0) |
| **Total** | **100** | **0** | ![](https://geps.dev/progress/0) |

---

## 🧠 Topics Covered

```
┌─────────────────────────────────────────────────────────────────┐
│                        DSA Topics                               │
├─────────────────┬─────────────────┬─────────────────────────────┤
│ Arrays          │ Strings         │ Linked Lists                │
│ Stacks          │ Queues          │ Hash Maps / Sets            │
│ Binary Search   │ Sorting         │ Two Pointers                │
│ Sliding Window  │ Recursion       │ Dynamic Programming         │
│ Backtracking    │ Graphs (DFS/BFS)│ Trees & Tries               │
│ Heaps           │ Greedy          │ Bit Manipulation            │
│ Union Find      │ Topological Sort│ System Design               │
└─────────────────┴─────────────────┴─────────────────────────────┘
```

---

## 📚 Resources

| Resource | Description |
|----------|-------------|
| [100 Rust Problems](https://github.com/aarambh-darshan/100-rust-problems) | Original problem repository |
| [Rust Book](https://doc.rust-lang.org/book/) | Official Rust documentation |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | Learn Rust with examples |
| [Rust Playground](https://play.rust-lang.org/) | Online Rust compiler |


