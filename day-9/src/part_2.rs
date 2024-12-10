#[derive(Debug)]
struct FileBlock {
    id: usize,
    start: usize,
    size: usize,
}

#[derive(Debug)]
struct FreeSpace {
    start: usize,
    size: usize,
}

pub fn process(input: &str) -> usize {
    let mut files: Vec<FileBlock> = Vec::new();
    let mut free_space: Vec<FreeSpace> = Vec::new();
    let mut free = false;
    let mut start = 0;
    let mut id = 0;

    for c in input.trim().chars() {
        let block_size = c.to_digit(10).unwrap() as usize;
        if free {
            free_space.push(FreeSpace {
                start,
                size: block_size,
            });
        } else {
            files.push(FileBlock {
                id,
                start,
                size: block_size,
            });
            id += 1;
        }
        free = !free;
        start += block_size;
    }

    for file in files.iter_mut().rev() {
        for free in free_space.iter_mut() {
            if free.size < file.size {
                continue;
            } else if file.start < free.start {
                break;
            }
            free.size -= file.size;
            file.start = free.start;
            free.start += file.size;
            break;
        }
    }

    files
        .iter()
        .map(|file| -> usize {
            let mut sum = 0;
            for i in file.start..file.start + file.size {
                sum += file.id * i;
            }
            sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        "2333133121414131402"
    }

    #[rstest]
    fn test_process(input: &str) {
        // when
        let output = process(input);

        // then
        assert_eq!(output, 2858)
    }
}
