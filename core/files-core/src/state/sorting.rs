use crate::models::FileEntry;

pub(crate) fn sort_entries(entries: &mut [FileEntry]) {
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn directories_come_before_files() {
        let mut entries = vec![
            FileEntry {
                name: "b.txt".into(),
                path: PathBuf::from("b.txt"),
                is_dir: false,
            },
            FileEntry {
                name: "a_dir".into(),
                path: PathBuf::from("a_dir"),
                is_dir: true,
            },
        ];

        sort_entries(&mut entries);

        assert!(entries[0].is_dir);
    }

    #[test]
    fn case_insensitive_sorting() {
        let mut entries = vec![
            FileEntry {
                name: "b.txt".into(),
                path: PathBuf::from("b.txt"),
                is_dir: false,
            },
            FileEntry {
                name: "A.txt".into(),
                path: PathBuf::from("A.txt"),
                is_dir: false,
            },
        ];

        sort_entries(&mut entries);

        assert_eq!(entries[0].name, "A.txt");
    }
}
