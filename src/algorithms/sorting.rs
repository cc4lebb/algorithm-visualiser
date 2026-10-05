
pub  struct BubbleSort {
    i: usize,
    j: usize, // set j to i + 1
    pub is_sorted: bool,
}

impl BubbleSort {
    pub fn new() -> Self {
        Self {
            i: 0, 
            j: 0, 
            is_sorted: false 
        }
    }
    pub fn step(&mut self, arr: &mut [f32]) -> bool { 
        let n = arr.len();
        let mut swapped = false;

        if n <= 1 || self.is_sorted {
            self.is_sorted = true;
            return false;
        }

        if arr[self.j] > arr[self.j + 1] {
            arr.swap(self.j, self.j + 1);
            swapped = true;
        }

        self.j += 1;

        if self.j >= n - 1 - self.i {
            self.j = 0;
            self.i += 1;

            if self.i >= n - 1 {
                self.is_sorted = true;
            }
        }

        swapped
    }
}

pub struct SelectionSort {
    i: usize,
    j: usize,
    pub is_sorted: bool,
}

impl SelectionSort {
    pub fn new() -> Self {
        Self {
            i: 0,
            j: 0,
            is_sorted: false
        }
    }

    pub fn step(&mut self, arr: &mut [f32]) -> bool {
        let n = arr.len();
        let mut swapped = false;

        let mut min_index = self.i;
            
        for j in (self.i + 1)..n {
            self.j = j;

            if arr[self.j] < arr[min_index] {
                min_index = self.j;
            }
        }

        if min_index != self.i {
            arr.swap(self.i, min_index);
            swapped = true; 
        }

        if self.j >= n - 1 - self.i {
            self.j = 0;
            self.i += 1;

            if self.i >= n - 1 {
                self.is_sorted = true;
            }
        }

        swapped
    
    }
}
