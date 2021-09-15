pub struct Crostab<'a, T> {
    head: Vec<&'a str>,
    side: Vec<&'a str>,
    rows: Vec<Vec<T>>,
}

impl<'a, T> Crostab<'a, T> {
    pub fn build(head: Vec<&'a str>, side: Vec<&'a str>, rows: Vec<Vec<T>>) -> Crostab<'a, T> {
        return Crostab { head, side, rows };
    }
}

#[cfg(test)]
mod tests {
    use spare::deco_matrix;

    use crate::crostab::Crostab;

    #[test]
    fn test_fluo_matrix_rendered() {
        let head = vec!["foo", "bar", "zen"];
        let side = vec!["2000", "1980", "1960"];
        let rows = vec![
            vec!["1", "2", "3"],
            vec!["1", "2", "3"],
            vec!["1", "2", "3"],
        ];
        let crostab = Crostab::build(head, side, rows);
        // crostab.rows.deco
        let texted = deco_matrix(crostab.rows, ", ");
        println!("rows = {}", texted);
    }
}