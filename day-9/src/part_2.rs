use std::collections::{HashMap, HashSet};

use crate::{convert_to_blocks, Blocks, GAP};

// 1. iterowac od poczatku bloku
// 2. iterowac od konca bloku
// 3. znalezc przerwe i obliczyc jej dlugosc
// 4. isc od tylu i sprawdzic jaki najwiekszy  indeks jest w stanie zmiescic sie w przerwie
// 5. pobrac wszystkie elementy dla tego indeksu i wstawic je w przerwe, na ich oryginalne miejsce wstawic GAP(s)
// 6. zakonczyc iteracje kiedy przejde po wszystkich przerwach ORAZ kiedy iteracja do przodu jest mniejsza od iteracji od konca
fn move_blocks_around(blocks: Blocks) -> Blocks {
    let mut blocks = blocks;
    let mut blocks_hashmap = HashMap::new();
    for &id in &blocks {
        *blocks_hashmap.entry(id).or_insert(0) += 1;
    }

    for i in 0..blocks.len() {
        let mut j = blocks.len() - 1;

        match blocks[i] {
            GAP => {
                let mut free_space = 1;
                for &id in blocks[i + 1..=j].iter() {
                    match id {
                        GAP => {
                            free_space += 1;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                println!("i: {i}: free space! length: {:?}", free_space);

                let mut ids_to_move = HashSet::new();
                while j > i {
                    let last_id = blocks[j];
                    let second_to_last_id = blocks[j - 1];
                    println!(
                        "\tj: {j}, last_id: {:?}, second_to_last: {:?}",
                        last_id, second_to_last_id
                    );
                    if last_id == GAP {
                        println!(
                            "\t\tgap found at blocks[{j}]: {:?}, continue search",
                            blocks[j]
                        );
                        j -= 1;
                        continue;
                    }

                    if last_id != second_to_last_id && ids_to_move.len() <= free_space {
                        println!("\t\tnext block is different, and ids fit into free space");
                        let mut tmp_i = i;
                        for id_to_move in ids_to_move {
                            println!(
                                "\t\t\treplacing blocks[{i}]: {:?} with blocks[{id_to_move}]: {:?}",
                                blocks[i], blocks[id_to_move as usize]
                            );
                            blocks[tmp_i] = blocks[id_to_move as usize];
                            blocks[id_to_move as usize] = GAP;
                            tmp_i += 1;
                        }

                        break;
                    }

                    if last_id == second_to_last_id {
                        ids_to_move.insert(j as i32);
                        ids_to_move.insert((j - 1) as i32);
                    } else {
                        ids_to_move = HashSet::new();
                    }

                    j -= 1;
                }
            }
            _ => {
                continue;
            }
        }
        println!("i: {i}, blocks: {:?}", blocks);
    }

    blocks
}

pub fn process(input: &str) -> usize {
    let blocks = convert_to_blocks(input.trim());
    let blocks = move_blocks_around(blocks);

    0
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
    fn test_move_blocks_around(input: &str) {
        // given
        let blocks = convert_to_blocks(input);

        // when
        let output = move_blocks_around(blocks);

        // then
        assert_eq!(
            output,
            [
                0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5,
                -1, 6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1,
            ]
        )
    }

    // #[rstest]
    // fn test_process(input: &str) {
    //     // when
    //     let output = process(input);

    //     // then
    //     assert_eq!(output, 2858)
    // }
}
