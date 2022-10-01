use crate::structs::MappingWithKey;

//json serde
fn handle_line(line: &str, row_num: i32) -> Result<MappingWithKey, String> {
    let columns: Vec<&str> = line.split('\t').collect();
    if columns.len() < 2 {
        return Err(format!("Invalid row {}", row_num));
    }
    Ok(MappingWithKey::new(columns[0], columns[1]))
}
pub(crate) fn handle_input(lines: &String) -> String {
    let mut row_num = 1;
    let (a, b): (Vec<_>, Vec<_>) = lines
        .split('\n')
        .skip(1)
        .map(|line| {
            row_num += 1;
            handle_line(line, row_num)
        })
        .partition(|i| i.is_ok());
    if !b.is_empty() {
        return b
            .into_iter()
            .map(|i| i.err().unwrap())
            .reduce(|accum, item| format!("{}\n{}", accum, item))
            .unwrap();
    }

    a.into_iter()
        .map(|r| r.unwrap())
        .map(|kwd| {
            format!(
                "\"{}\":{},\n",
                kwd.ecwid_id,
                serde_json::to_string(&kwd.details).unwrap()
            )
        })
        .reduce(|accum, item| format!("{}{}", accum, item))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::handling;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_produces_output() {
        let input_value = String::from("A\tB\n123\t881881sds\n611\t9902");
        let result = handling::handle_input(&input_value);
        assert_eq!(
            result,
            String::from("\"123\":{\"d\":\"881881sds\"},\n\"611\":{\"d\":\"9902\"},\n")
        );
    }

    #[test]
    fn it_handles_error() {
        let input_value = String::from("A\tB\n123\t881881sds\n5\n611\t9902");
        let result = handling::handle_input(&input_value);
        assert_eq!(result, String::from("Invalid row 3"));
    }
}
