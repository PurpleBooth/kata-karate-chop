# Kata: Karate Chop

## Challenge

[Karate Chop](http://codekata.com/kata/kata02-karate-chop/) Kata from [codekata](http://codekata.com/kata/kata02-karate-chop/)

     

Specification

> Write a binary chop method that takes an integer search target and a sorted array of integers. It should return the integer index of the target in the array, or -1 if the target is not in the array. The signature will logically be:
>
> ```ruby
> chop(int, array_of_int)  -> int
> ```
> 
> You can assume that the array has less than 100,000 elements. For the purposes of this Kata, time and memory performance are not issues (assuming the chop terminates before you get bored and kill it, and that you have enough RAM to run it).

## Test Data

```ruby
def test_chop
  assert_equal(-1, chop(3, []))
  assert_equal(-1, chop(3, [1]))
  assert_equal(0,  chop(1, [1]))
  #
  assert_equal(0,  chop(1, [1, 3, 5]))
  assert_equal(1,  chop(3, [1, 3, 5]))
  assert_equal(2,  chop(5, [1, 3, 5]))
  assert_equal(-1, chop(0, [1, 3, 5]))
  assert_equal(-1, chop(2, [1, 3, 5]))
  assert_equal(-1, chop(4, [1, 3, 5]))
  assert_equal(-1, chop(6, [1, 3, 5]))
  #
  assert_equal(0,  chop(1, [1, 3, 5, 7]))
  assert_equal(1,  chop(3, [1, 3, 5, 7]))
  assert_equal(2,  chop(5, [1, 3, 5, 7]))
  assert_equal(3,  chop(7, [1, 3, 5, 7]))
  assert_equal(-1, chop(0, [1, 3, 5, 7]))
  assert_equal(-1, chop(2, [1, 3, 5, 7]))
  assert_equal(-1, chop(4, [1, 3, 5, 7]))
  assert_equal(-1, chop(6, [1, 3, 5, 7]))
  assert_equal(-1, chop(8, [1, 3, 5, 7]))
end
```
 
# Running

Format code

```
rustup run nightly cargo fmt --all --
```

Run tests

```
$  rustup run stable cargo test             master ✚ ✱
     Compiling kata-karate-chop v0.1.0 (file:///Users/billie/Documents/Code/kata-karate-chop)
      Finished dev [unoptimized + debuginfo] target(s) in 1.23 secs
       Running target/debug/deps/katakaratechop-635aa9985c5a2d1d
  
  running 40 tests
  test tests::case_iterative_01 ... ok
  test tests::case_iterative_02 ... ok
  test tests::case_iterative_03 ... ok
  test tests::case_iterative_04 ... ok
  test tests::case_iterative_05 ... ok
  test tests::case_iterative_06 ... ok
  test tests::case_iterative_07 ... ok
  test tests::case_iterative_08 ... ok
  test tests::case_iterative_09 ... ok
  test tests::case_iterative_10 ... ok
  test tests::case_iterative_11 ... ok
  test tests::case_iterative_12 ... ok
  test tests::case_iterative_13 ... ok
  test tests::case_iterative_14 ... ok
  test tests::case_iterative_15 ... ok
  test tests::case_iterative_16 ... ok
  test tests::case_iterative_17 ... ok
  test tests::case_iterative_18 ... ok
  test tests::case_iterative_19 ... ok
  test tests::case_iterative_20 ... ok
  test tests::case_stack_01 ... ok
  test tests::case_stack_02 ... ok
  test tests::case_stack_03 ... ok
  test tests::case_stack_04 ... ok
  test tests::case_stack_05 ... ok
  test tests::case_stack_06 ... ok
  test tests::case_stack_07 ... ok
  test tests::case_stack_08 ... ok
  test tests::case_stack_09 ... ok
  test tests::case_stack_10 ... ok
  test tests::case_stack_11 ... ok
  test tests::case_stack_12 ... ok
  test tests::case_stack_13 ... ok
  test tests::case_stack_14 ... ok
  test tests::case_stack_15 ... ok
  test tests::case_stack_16 ... ok
  test tests::case_stack_17 ... ok
  test tests::case_stack_18 ... ok
  test tests::case_stack_19 ... ok
  test tests::case_stack_20 ... ok
  
  test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  
     Doc-tests katakaratechop
  
  running 0 tests
  
  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 


```
