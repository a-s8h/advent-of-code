fn input_to_file_map(input: &str) -> Vec<Option<u64>> {
    let mut file_map: Vec<Option<u64>> = Vec::with_capacity(input.len() * 9);
    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap();
        if i % 2  == 0 {
            for _ in 0..count {
                file_map.push(Some((i / 2) as u64));      
            }
        } else {
            for _ in 0..count {
                file_map.push(None);      
            }
        }
    }
    file_map
}

fn move_blocks(file_map: &mut Vec<Option<u64>>) -> () {
    let mut free_pos:usize = 0;
    let mut last_block_pos = file_map.len() - 1;
    loop {
        while file_map[free_pos].is_some() {
            free_pos += 1;
        }
        while file_map[last_block_pos].is_none() {
            last_block_pos -= 1;
        }

        if free_pos >= last_block_pos {
            break;
        }
        file_map.swap(free_pos, last_block_pos);
    }
}

fn calculate_check_sum(file_map: &Vec<Option<u64>>) -> u64 {
    let mut check_sum: u64 = 0;
    for (i, block) in file_map.iter().enumerate() {
        if let Some(id) = block {
            check_sum += (i as u64) * id;
        }
    }
    check_sum
}

#[derive(Debug)]
struct Chunk {
    uncompressed_index: usize,
    count: usize,
    file_id: usize,
}

pub fn part_2(input: &str) -> usize {
    let high_index: usize = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();

    let uncompressed_reversed =
        (0..input.len()).rev().zip(input.chars().rev());
    let reverse = uncompressed_reversed
        .scan(
            high_index,
            |base_index, (compressed_index, c)| {
                let num_indices =
                    c.to_digit(10).unwrap() as usize;
                *base_index -= num_indices;

                if compressed_index % 2 == 0 {
                    Some(Some(Chunk {
                        uncompressed_index: *base_index,
                        count: num_indices,
                        file_id: compressed_index as usize
                            / 2,
                    }))
                } else {
                    Some(None)
                }
            },
        )
        .flatten();
    // .filter_map(|v| v);

    // (uncompressed_index, space_count)
    let mut empties = input
        .chars()
        .enumerate()
        .fold(
            (0, vec![]),
            |(mut uncompressed_index, mut empties),
             (compressed_index, c)| {
                let num_indices =
                    c.to_digit(10).unwrap() as usize;
                if compressed_index % 2 != 0 {
                    empties.push((
                        uncompressed_index,
                        num_indices,
                    ))
                }
                uncompressed_index += num_indices;
                (uncompressed_index, empties)
            },
        )
        .1;

    // let mut moved_ids: Vec<usize> = vec![];
    let mut moved_chunks: Vec<Chunk> = vec![];
    for chunk in reverse {
        let Some(empty) =
            empties.iter_mut().find(|(i, empty_space)| {
                chunk.count <= (*empty_space as usize)
                    && *i < chunk.uncompressed_index
            })
        else {
            continue;
        };

        // moved_ids.push(chunk.file_id);
        moved_chunks.push(Chunk {
            uncompressed_index: empty.0 as usize,
            ..chunk
        });
        empty.0 += chunk.count;
        empty.1 -= chunk.count;
    }

    let mut base_index = 0;
    let mut sum = 0;
    // let mut last_uncompressed_index = usize::MAX;

    for (compressed_index, c) in input.chars().enumerate() {
        let num_indices = c.to_digit(10).unwrap() as usize;
        let file_id = compressed_index / 2;

        for uncompressed_index in
            base_index..(base_index + num_indices)
        {
            if compressed_index % 2 == 0
                && !moved_chunks.iter().any(|chunk| {
                    chunk.file_id == file_id as usize
                })
            {
                sum += uncompressed_index * file_id;
            }
        }

        base_index += num_indices;
    }

    for chunk in moved_chunks.iter() {
        for index in chunk.uncompressed_index
            ..(chunk.uncompressed_index + chunk.count)
        {
            sum += index * chunk.file_id;
        }
        // sum +=    chunk.
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day9.txt").unwrap();
        let mut file_map = input_to_file_map(&input.trim());
        move_blocks(&mut file_map);
        assert_eq!(calculate_check_sum(&file_map), 6307275788409);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day9.txt").unwrap();
        assert_eq!(part_2(&input.trim()), 6327174563252);
    }
}