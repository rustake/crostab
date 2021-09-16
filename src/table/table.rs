use veho::vector::index_of;

pub struct Table<T> {
    head: Vec<String>,
    rows: Vec<Vec<T>>,
}

impl<T> Table<T> {
    pub fn build(head: Vec<String>, rows: Vec<Vec<T>>) -> Table<T> {
        return Table { head, rows };
    }
    fn height(&self) -> usize { (&self.head).len() }
    fn width(&self) -> usize { (&self.rows).len() }
    fn coin(&self, head: &str) -> Option<usize> { index_of(&self.head, &head.to_string()) }
    fn cell(&self, side_index: usize, head_label: &str) -> Option<&T> {
        match self.coin(head_label) {
            Some(head_index) => Some(&self.rows[side_index][head_index]),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use spare::deco_matrix;
    use veho::matrix::mapper as matrix_mapper;
    use veho::vector::Mappers;

    use crate::crostab::Crostab;
    use crate::Table;

    #[test]
    fn test_crostab_simplified() {
        let head = vec!["foo", "bar", "zen"].mapper(|x| x.to_string());
        let rows = matrix_mapper(vec![
            vec!["1", "2", "3"],
            vec!["1", "2", "3"],
            vec!["1", "2", "3"],
        ], |x| x.to_string());
        let crostab = Table::build(head, rows);

        let texted = deco_matrix(&(&crostab).rows, ", ");
        println!("rows = {}", &texted);
        println!("height = {}, width = {}", (&crostab).height(), (&crostab).width());
        let side_label = "1984";
        let head_label = "bar";
        let cell = (&crostab).cell(2, head_label);

        println!("cell = {}", cell.unwrap_or(&("NA".to_string())))
    }
}