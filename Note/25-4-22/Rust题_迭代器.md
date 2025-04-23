### Rust练习题

1. fn main() {
       let v1 = vec![1, 2];

   ```rust
   // borrowing
   let mut v1_iter = v1.iter();
   
   assert_eq!(v1_iter.next(),Some(&1));
   assert_eq!(v1_iter.next(),Some(&2));
   assert_eq!(v1_iter.next(),None);
   ```
   }

   - 只有实现iterators trait 才能使用next方法，使用into_iter()将Vector类型转换为 iterator
   - into_iter :convert collections into iterators
   - fn next(&mut self)->Option<>;

2. ##### into_iter iter and iter_mut

   - into_iter: get the ownership,no longer available for reuse

   - iter:available for reuse after loop

   - iter_mut:mutably borrow ,can be modified in place

     - fn main() {
           let mut values = vec![1, 2, 3];
           let mut values_iter = values.iter_mut();

       ```rust
       if let Some(v) = values_iter.next() {
           *v = 0;
       }
       
       assert_eq!(values, vec![0, 2, 3]);
       ```
       }

3. 迭代器的使用，for与迭代器进行对比

   - ```rust
     fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
         let mut count = 0;
         for val in map.values() {
             if val == &value {
                 count += 1;
             }
         }
         count
     }
     
     fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
         map.values().filter(|&v|*v==value).count()
     }
     ```

   - ```rust
     fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
         let mut count = 0;
         for map in collection {
             for val in map.values() {
                 if val == &value {
                     count += 1;
                 }
             }
         }
         count
     }
     
     fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
       collection.iter().flat_map(|map|map.values()).filter(|&v|*v==value).count()
     }
     ```
   
3. 迭代器方法

   - `map`:转换数据，接受一个闭包并未迭代器中的每个元素调用该闭包，然后返回一个新的迭代器
   
   - ```rust
     let v = vec![1, 2, 3];
     let v_squared: Vec<i32> = v.iter().map(|x| x * x).collect();
     ```
   
   - `filter`：过滤数据。接受一个闭包并为迭代器中的每个元素调用该闭包。如果闭包返回true，则元素将包含在新的迭代器中。
   
   - ```rust
     let v = vec![1, 2, 3];
     let v_even: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
     ```
   
   - `fold`：聚合数据。接受一个初始值和一个闭包，并将闭包应用于初始值和迭代器中的每个元素，以生成一个单一的最终值。

   - ```rust
     let v = vec![1, 2, 3];
     let sum: i32 = v.iter().fold(0, |acc, x| acc + x);
     ```
   
   - `chain`：该方法是`Iterator` trait的一个方法，它允许你将两个迭代器链接在一起，形成一个新的迭代器。这个新的迭代器会先遍历第一个迭代器中的所有元素，然后再遍历第二个迭代器中的所有元素
   
   - ```rust
     let a = [1, 2, 3];
     let b = [4, 5];
     let c: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
     assert_eq!(c, [1, 2, 3, 4, 5]);
     ```
   
   - `take(n)`:获取前n个元素
   
   - `take_while()`:获取元素直到满足条件
   
   - `skip()`:跳过前n个元素
   
   - `enumerate`
   
   - `zip`
   
   - `cloned`
   
5. 要解决这个问题，我们需要理解 `Cow`（Clone-On-Write）类型在不同情况下的行为，并根据是否发生克隆来验证测试结果。`Cow` 可以是 `Borrowed`（引用外部数据）或 `Owned`（拥有数据的所有权），当需要修改数据时，如果数据是借用的，`Cow` 会克隆数据变为 `Owned`。

   ### 分析
   1. **reference_mutation**：传入的切片包含负数，需要修改，因此 `Cow` 变为 `Owned`。
   2. **reference_no_mutation**：传入的切片没有负数，无需修改，保持 `Borrowed`。
   3. **owned_no_mutation**：传入 `Vec`，`Cow` 直接拥有数据且无需修改，保持 `Owned`。
   4. **owned_mutation**：传入包含负数的 `Vec`，直接修改原有数据，保持 `Owned`。

   ### 解决方案
   根据上述分析，补充测试中的 `match` 表达式以检查正确的 `Cow` 状态。

   ```rust
   // cow1.rs
   use std::borrow::Cow;
   
   fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
       for i in 0..input.len() {
           let v = input[i];
           if v < 0 {
               input.to_mut()[i] = -v;
           }
       }
       input
   }
   
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn reference_mutation() -> Result<(), &'static str> {
           let slice = [-1, 0, 1];
           let mut input = Cow::from(&slice[..]);
           match abs_all(&mut input) {
               Cow::Owned(_) => Ok(()),
               _ => Err("Expected owned value"),
           }
       }
   
       #[test]
       fn reference_no_mutation() -> Result<(), &'static str> {
           let slice = [0, 1, 2];
           let mut input = Cow::from(&slice[..]);
           match abs_all(&mut input) {
               Cow::Borrowed(_) => Ok(()),
               _ => Err("Expected borrowed value"),
           }
       }
   
       #[test]
       fn owned_no_mutation() -> Result<(), &'static str> {
           let slice = vec![0, 1, 2];
           let mut input = Cow::from(slice);
           match abs_all(&mut input) {
               Cow::Owned(_) => Ok(()),
               _ => Err("Expected owned value"),
           }
       }
   
       #[test]
       fn owned_mutation() -> Result<(), &'static str> {
           let slice = vec![-1, 0, 1];
           let mut input = Cow::from(slice);
           match abs_all(&mut input) {
               Cow::Owned(_) => Ok(()),
               _ => Err("Expected owned value"),
           }
       }
   }
   ```

   ### 说明
   - **reference_mutation**：由于存在负数，`to_mut()` 克隆数据，`Cow` 变为 `Owned`。
   - **reference_no_mutation**：无修改，保持 `Borrowed`。
   - **owned_no_mutation**：数据已为 `Owned`，无需克隆，保持 `Owned`。
   - **owned_mutation**：数据已为 `Owned`，直接修改，保持 `Owned`。

   每个测试验证了 `Cow` 在克隆和直接使用所有权时的正确行为。