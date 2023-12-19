use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    let filters = arguments[2..].to_vec();

    let lines = read_file_lines_to_vector(&arguments[1]).unwrap();
    let relevance_index = lines
        .iter()
        .enumerate()
        .filter(|(_, s)| s.contains("set current project to"))
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .0
        + 1;

    let relevant_lines: Vec<String> = lines[relevance_index..]
        .to_vec()
        .iter_mut()
        .map(|s| s.replace("[info] ", ""))
        .collect();

    // println!("Starting the parse with line {}", relevant_lines[0]);

    let breadcrumbs = create_breadcrumbs(relevant_lines)
        .into_iter()
        .filter(|b| filters.iter().any(|f| b.name.contains(f)))
        .collect::<Vec<_>>();
    breadcrumbs_matrix(breadcrumbs);
}

fn breadcrumbs_matrix(breadcrumbs: Vec<Breadcrumb>) {
    let max_length = breadcrumbs
        .iter()
        .map(|breadcrumb| breadcrumb.previous.len())
        .max()
        .unwrap()
        + 1;

    let mut matrix: Vec<Vec<String>> = vec![];
    for breadcrumb in breadcrumbs {
        let mut row = vec![];
        for pbc in breadcrumb.previous {
            row.push(pbc);
        }
        row.push(breadcrumb.name);
        matrix.push(row);
    }

    for index in 0..max_length {
        let mut track = String::new();
        for row in matrix.iter_mut() {
            if row.len() > index {
                if row[index] == track {
                    row[index] = "".to_string();
                } else {
                    track = row[index].clone();
                }
            }
        }
    }

    for row in matrix {
        let mut non_empty_count = 0;
        for s in row.iter() {
            if !s.is_empty() {
                non_empty_count += 1;
            }
        }
        if non_empty_count > 1 {
            let mut buf = String::new();
            for s in row.iter() {
                let str = format!("{}{}", buf, s);
                if index_of_first_letter(&str).is_some() {
                    println!("{}", str);
                }
                buf.push_str(" | ");
            }
            continue;
        } else {
            let str = row.join(" | ");
            if index_of_first_letter(&str).is_some() {
                println!("{}", str);
            }
        }
    }
}

fn read_file_lines_to_vector(path: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    let contents = String::from_utf8_lossy(&contents);
    let lines = contents.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

fn index_of_first_letter(s: &str) -> Option<usize> {
    s.chars().position(|c| c.is_alphabetic())
}

fn create_breadcrumbs(lines: Vec<String>) -> Vec<Breadcrumb> {
    let mut dep_with_breadcrumbs = vec![];
    let mut breadcrumbs = vec![];
    let mut previous_index = 0;

    for line in lines {
        //based on the index of the first letter I can determine the depth of the breadcrumb
        //relative to the previous breadcrumb
        let index_op = index_of_first_letter(&line);
        if index_op.is_none() {
            continue;
        }
        let index = index_op.unwrap();
        let name = line[index..].to_string();

        if index == 0 {
            dep_with_breadcrumbs.clear();
            dep_with_breadcrumbs.push(Breadcrumb::new(vec![], name.clone()));
            breadcrumbs.push((index, name));
            previous_index = 0;
            continue;
        } else if index < previous_index {
            //it's on a shallower level than the previous breadcrumb, I will drop all breadcrumbs
            //deeper than this one
            breadcrumbs = breadcrumbs
                .into_iter()
                .filter(|(i, _)| *i < index)
                .collect();
            breadcrumbs.push((index, name.clone()));
            dep_with_breadcrumbs.push(Breadcrumb::new(
                breadcrumbs
                    .clone()
                    .iter()
                    .filter(|(i, _)| *i < index)
                    .map(|(_, name)| name.clone())
                    .collect(),
                name.clone(),
            ));
            previous_index = index;
            continue;
        } else if index == previous_index {
            //I am done with the previous breadcrumb, it has no more children
            breadcrumbs.pop();
            breadcrumbs.push((index, name.clone()));
            dep_with_breadcrumbs.push(Breadcrumb::new(
                breadcrumbs
                    .clone()
                    .iter()
                    .filter(|(i, _)| *i < index)
                    .map(|(_, name)| name.clone())
                    .collect(),
                name.clone(),
            ));
            continue;
        } else if index > previous_index {
            //it's a child of the previous breadcrumb
            breadcrumbs.push((index, name.clone()));
            dep_with_breadcrumbs.push(Breadcrumb::new(
                breadcrumbs
                    .clone()
                    .iter()
                    .filter(|(i, _)| *i < index)
                    .map(|(_, name)| name.clone())
                    .collect(),
                name.clone(),
            ));
            previous_index = index;
            continue;
        }
    }
    dep_with_breadcrumbs
}

#[derive(Debug, Clone)]
struct Breadcrumb {
    previous: Vec<String>,
    name: String,
}

impl Breadcrumb {
    fn new(previous: Vec<String>, name: String) -> Self {
        Self { previous, name }
    }
}
