pub mod csv {

    enum Direction {
        Row,
        Column,
    }

    pub struct CSV {
        pub content: Vec<Vec<String>>,
    }

    impl CSV {
        pub fn new(source: String) -> Self {
            let mut s = Self {
                content: Vec::<Vec<String>>::new(),
            };
            s.parse(source);
            s
        }

        pub fn get_row(&self, row: usize) -> &Vec<String> {
            return &self.content[row];
        }

        pub fn get_column_by_header(&self, header: &str) -> Vec<String> {
            let mut header_idx: usize = 0;

            for i in 0..self.content[0].len() {
                if self.content[0][i] == header {
                    header_idx = i;
                }
            }
            return self.get_column(header_idx);
        }

        pub fn get_column(&self, column: usize) -> Vec<String> {
            let mut col: Vec<String> = Vec::<String>::new();
            for i in self.content[1..].iter() {
                col.push(String::from(&i[column]));
            }
            return col;
        }

        pub fn value_at(&self, row: usize, column: usize) -> &String {
            return &self.content[row][column];
        }

        pub fn parse(&mut self, source: String) {
            let lines: Vec<&str> = source.split_terminator("\n").collect();

            for i in lines.iter() {
                let mut v: Vec<String> = Vec::<String>::new();
                for j in i.split(",") {
                    v.push(String::from(j));
                }
                self.content.push(v);
            }
        }

        pub fn sum_row(&self, index: usize) -> u32 {
            return self.sum(Direction::Row, index);
        }

        pub fn sum_column(&self, index: usize) -> u32 {
            return self.sum(Direction::Column, index);
        }

        fn sum(&self, direction: Direction, index: usize) -> u32 {
            let mut sum: u32 = 0;
            match direction {
                Direction::Row => {
                    let row = self.get_row(index);
                    for i in row {
                        match i.parse::<u32>() {
                            Err(why) => panic!("Error: {}", why),
                            Ok(num) => sum += num,
                        }
                    }
                }
                Direction::Column => {
                    let column = self.get_column(index);
                    for i in column {
                        match i.parse::<u32>() {
                            Err(why) => panic!("Error: {}", why),
                            Ok(num) => sum += num,
                        }
                    }
                }
            }
            return sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::csv::CSV;

    fn create_fixture() -> CSV {
        let s = String::from("a,b,c,d\n1,2,3,4\n5,6,7,8\n9,10,11,12,13");

        CSV::new(s)
    }

    #[test]
    fn test_new() {
        let fixture = create_fixture();
        assert_eq!(fixture.content.len(), 4);
        assert_eq!(fixture.content[0].len(), 4);
    }

    #[test]
    fn test_parse() {
        let s = String::from("a,b,c,d\n1,2,3,4\n5,6,7,8\n9,10,11,12,13");

        let mut fixture = CSV { content: Vec::<Vec<String>>::new() };
        fixture.parse(s);

        assert_eq!(fixture.content.len(), 4);
        assert_eq!(fixture.content[0].len(), 4);
    }

    #[test]
    fn test_get_row() {
        let fixture = create_fixture();

        let row = fixture.get_row(2);

        let mut row_actual = Vec::<String>::new();
        row_actual.push(String::from("5"));
        row_actual.push(String::from("6"));
        row_actual.push(String::from("7"));
        row_actual.push(String::from("8"));
        assert_eq!(row, &row_actual);
    }

    #[test]
    fn test_get_column() {
    }
}
