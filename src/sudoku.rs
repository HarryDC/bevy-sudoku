use array2d::Array2D;
use std::fs;
use std::vec;

#[derive(Debug, Clone)]
pub struct Field {
    pub value : Option<i32>,
    pub is_original : bool,
    pub options : Vec<i32>
}

impl Field {
    fn fixed(value: i32) -> Field {
        return Field{value: Some(value), is_original: true, options: vec![] }
    }

    fn remove(&mut self, val : i32) {
        if let Some(index) = self.options.iter().position(|&x| x == val) {
            self.options.remove(index);            
        }
    }

    fn fix(&mut self, val : i32) {
        self.value = Some(val);
        self.options.clear();
    }

    fn fix_if_unique(&mut self) -> bool {
        if self.options.len() == 1 {
            self.fix(self.options[0]);
            return true;
        } else {
            return self.options.len() == 0;
        }
    }
}

impl Default for Field { 
    fn default() -> Self {
        return Field{value: None, is_original: false, options:vec![1,2,3,4,5,6,7,8,9]};
    }
}

#[cfg(test)]
mod FieldTests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fix() {
        let mut f = Field::default();
        f.fix(1);
        assert_eq!(f.value, Some(1));
        assert_eq!(f.options.len(), 0);
    }

    #[test]
    fn test_remove() {
        let mut f: Field = Field{options: vec![1,2,3,4], ..Default::default()};
        assert_eq!(f.options, vec![1,2,3,4]);
        f.remove(5);
        assert_eq!(f.options, vec![1,2,3,4]);
        f.remove(1);
        assert_eq!(f.options, vec![2,3,4]);
    }
}

fn print_array(array : &Array2D<Field>) {
    for row_iter in array.rows_iter() {
        for element in row_iter {
            print!("{} ", element.value.unwrap_or(0));
        }
        println!();
    }
}

pub fn update(data : &mut Array2D<Field>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if data[(i,j)].value == None {
                update_from_block(data, (i,j));
                update_from_row_and_column(data, (i,j));
            }
        }
    }
    let mut done = true;
    for i in 0..9 {
        for j in 0..9 {
            done = data[(i,j)].fix_if_unique() && done;
        }
    }
    return done;
}

fn update_from_row_and_column(data: &mut Array2D<Field>, address: (usize, usize)) {
    for i in 0..9 {
        if let Some(value) = data[(i,address.1)].value {
            data[address].remove(value);
        }
        if let Some(value) = data[(address.0, i)].value {
            data[address].remove(value);
        }
    }
}

fn update_from_block(data : &mut Array2D<Field>, address : (usize, usize)) {
    let block_i = address.0/3 * 3;
    let block_j = address.1/3 * 3;

    for i in block_i..block_i+3 {
        for j in block_j..block_j+3 {
            let block_val = &data[(i,j)].value;
            if address == (i,j) || block_val.is_none() {
                continue;
            } else {
                let block_val = block_val.unwrap();
                data[address].remove(block_val);
            }
        }
    }
}

pub fn read_puzzle(path : &str) -> Array2D<Field> {
    let mut data: Array2D<Field> = Array2D::filled_with(Field::default(), 9 , 9);
    let contents = fs::read_to_string(path).unwrap();
    let mut x = 0;
    for line in contents.lines()  {
        let mut y = 0;
        for char in line.split(',') {
            match char.parse::<i32>() {
              Ok(value) => {
                if value > 0 {
                    data[(x,y)] = Field::fixed(value)
                } 
              },
              Err(_) => {}
            }
            y+=1;
        }
        x+=1;
    }

    return data
}

fn print_field(puzzle: &Array2D<Field>, index: (usize, usize)) {
    let field = &puzzle[index];
    println!("{index:?} : {field:?}");
}

