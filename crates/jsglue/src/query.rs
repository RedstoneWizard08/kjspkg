pub fn parse_query(inp: &str) -> Vec<(String, String)> {
    let mut res = Vec::new();
    let spl = inp.split('&');

    for item in spl {
        let item = item.trim_start_matches('?');
        let mut spl = item.split('=').collect::<Vec<&str>>();

        spl.reverse();

        let name = spl.pop().unwrap().to_string();

        spl.reverse();

        let value = spl.join("=");

        res.push((name, value));
    }

    res
}
