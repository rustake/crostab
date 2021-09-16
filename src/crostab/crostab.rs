use veho::vector::index_of;

pub struct Crostab<T> {
    head: Vec<String>,
    side: Vec<String>,
    rows: Vec<Vec<T>>,
}

impl<T> Crostab<T> {
    pub fn build(head: Vec<String>, side: Vec<String>, rows: Vec<Vec<T>>) -> Crostab<T> {
        return Crostab { head, side, rows };
    }
    fn height(&self) -> usize { (&self.head).len() }
    fn width(&self) -> usize { (&self.side).len() }
    fn roin(&self, side: &str) -> Option<usize> { index_of(&self.side, &side.to_string()) }
    fn coin(&self, head: &str) -> Option<usize> { index_of(&self.head, &head.to_string()) }
    fn cell(&self, side: &str, head: &str) -> Option<&T> {
        match (self.roin(side), self.coin(head)) {
            (Some(xi), Some(yi)) => Some(&self.rows[xi][yi]),
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

    #[test]
    fn test_crostab_simplified() {
        let side = vec!["2004", "1984", "1964"].mapper(|x| x.to_string());
        let head = vec!["foo", "bar", "zen"].mapper(|x| x.to_string());
        let rows = matrix_mapper(vec![
            vec!["1", "2", "3"],
            vec!["1", "2", "3"],
            vec!["1", "2", "3"],
        ], |x| x.to_string());
        let crostab = Crostab::build(head, side, rows);

        let texted = deco_matrix(&(&crostab).rows, ", ");
        println!("rows = {}", &texted);
        println!("height = {}, width = {}", (&crostab).height(), (&crostab).width());
        let side_label = "1984";
        let head_label = "bar";
        let cell = (&crostab).cell(side_label, head_label);

        println!("cell = {}", cell.unwrap_or(&("NA".to_string())))
    }
}