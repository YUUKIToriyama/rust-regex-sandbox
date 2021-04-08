use regex::Regex;

fn main() {
	let str = "2021-04-08";
	let re = Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
	let caps = re.captures(str).unwrap();
	println!("{}", &caps["year"]);
}