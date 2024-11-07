#[derive(Debug)]
pub struct FixedPriorityQueue<T, const N: usize> {
    arr: [T; N],
    size: usize,
}

impl<T, const N: usize> FixedPriorityQueue<T, N>
where
    T: Default + Copy + PartialOrd,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    pub fn insert(&mut self, val: T) -> Result<usize, String> {
        if self.size >= N {
            return Err(format!("queue full with capacity {}", N));
        }

        self.arr[self.size] = val;
        self.bubble_up(self.size);
        self.size += 1;

        return Ok(self.size - 1);
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let result = Some(self.arr[0]);

        self.arr[0] = self.arr[self.size - 1];
        self.size -= 1;
        self.bubble_down(0);

        return result;
    }

    fn bubble_up(&mut self, current: usize) {
        let parent = if current == 0 { 0 } else { (current - 1) >> 1 };
        if current >= 1 && self.arr[current] > self.arr[parent] {
            self.arr.swap(current, parent);
            self.bubble_up(parent);
        }
    }

    fn bubble_down(&mut self, current: usize) {
        let left = (current << 1) + 1;
        let right = (current << 1) + 2;
        let mut max = current;

        if left < self.size && self.arr[left] > self.arr[max] {
            max = left;
        }

        if right < self.size && self.arr[right] > self.arr[max] {
            max = right;
        }

        if max != current {
            self.arr.swap(current, max);
            self.bubble_down(max);
        }
    }
}

impl<T, const N: usize> Default for FixedPriorityQueue<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        FixedPriorityQueue {
            arr: [T::default(); N],
            size: 0,
        }
    }
}

impl<T, const N: usize> From<[T; N]> for FixedPriorityQueue<T, N>
where
    T: Default + Copy + PartialOrd,
{
    fn from(value: [T; N]) -> Self {
        let mut queue = FixedPriorityQueue::<T, N> {
            arr: value,
            size: value.len(),
        };

        for i in (0..value.len() / 2).rev() {
            queue.bubble_down(i);
        }

        return queue;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut queue = FixedPriorityQueue::<i32, 15>::new();
        let arr = [98, 86, 41, 13, 65, 32, 29, 9, 10, 44, 23, 21, 17];

        for (index, val) in arr.iter().enumerate() {
            queue.arr[index] = *val;
        }
        queue.size = arr.len();

        queue.insert(81).unwrap();

        assert!(
            format!("{:?}", queue.arr)
                == "[98, 86, 81, 13, 65, 32, 41, 9, 10, 44, 23, 21, 17, 29, 0]"
        );
    }

    #[test]
    fn remove() {
        let mut queue = FixedPriorityQueue::<i32, 15>::new();
        let arr = [98, 86, 41, 13, 65, 32, 29, 9, 10, 44, 23, 21, 17];

        for (index, val) in arr.iter().enumerate() {
            queue.arr[index] = *val;
        }
        queue.size = arr.len();

        queue.remove();

        assert!(
            format!("{:?}", queue.arr)
                == "[86, 65, 41, 13, 44, 32, 29, 9, 10, 17, 23, 21, 17, 0, 0]"
        );
    }

    #[test]
    fn heapify() {
        let arr = [89, 29, 23, 36, 48, 94, 13, 27, 70, 76, 37, 42, 58];
        let queue: FixedPriorityQueue<_, 13> = arr.into();

        assert!(
            format!("{:?}", queue.arr) == "[94, 76, 89, 70, 48, 58, 13, 27, 36, 29, 37, 42, 23]"
        );
    }
}
