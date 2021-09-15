pub struct Table<'a, T> {
    head: Vec<&'a str>,
    rows: Vec<Vec<T>>,
}

#[cfg(test)]
mod tests {}