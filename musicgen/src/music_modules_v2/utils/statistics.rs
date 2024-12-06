//! A module for computing the probabilities of chord choices.

use std::collections::HashSet;

use crate::music_modules_v2::{chord::Chord, Music};

use super::sets::SetMath;


impl Music {
    /// Sets the probabilities of chords being picked within self.chord_table 
    /// and self.chord_list.
    #[allow(unused)]
    pub fn set_probabilities(&mut self) {
        let mut chord_table_set: Vec<HashSet<Chord>> = self.chord_table
            .iter()
            .map(|col| col.iter().cloned().collect::<HashSet<Chord>>())
            .collect();
        let filled_cols = chord_table_set
            .iter()
            .filter(|col| !col.is_empty())
            .count();

        let mut visited_chords_set: HashSet<Chord> = HashSet::with_capacity(self.chord_list.len());
        let probability_1d = 1.0 / self.chord_list.len() as f32;

        for (col_index, column) in self.chord_table.iter().enumerate() {
            if column.is_empty() {
                continue;
            }

            for mut chord in column.iter().cloned() {
                if visited_chords_set.contains(&chord) {
                    continue;
                }

                let mut appearances = Vec::with_capacity(9);
                appearances.push(col_index);
                for (c_idx, other_col) in chord_table_set.iter().enumerate().skip(col_index + 1) {
                    if other_col.contains(&chord) {
                        appearances.push(c_idx);
                    }
                }

                let mut probability = 0.0;
                for idx in appearances.iter() {
                    probability += (1.0 / filled_cols as f32) * (1.0 / chord_table_set[*idx].len() as f32);
                }

                chord.show_probability = true;
                chord.probability_2d = probability;
                chord.probability_1d = probability_1d;

                for idx in appearances {
                    chord_table_set[idx].replace(chord.clone());
                }
                visited_chords_set.insert(chord.clone());
            }
        }

        for (vec, set) in self.chord_table.iter_mut().zip(chord_table_set.iter()) {
            *vec = set.to_vec()
        }

        self.chord_list = visited_chords_set.to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_probabilities_test() {
        let key = "Cmin";
        let scale = "disabled";
        let mut musician = Music::smoke_hash(Default::default(), &key, &HashSet::new(), "default", &scale, true, false).unwrap();
        //let mut musician = Music::smoke_hash_all_pruning_chords(key, &scale);
        
        let table_scheme = "contains_note";

        match table_scheme {
            "contains_note" => musician.rotate_chords("F#min"),
            "highest_note" => musician.rearrange_by_highest_note("F#min"),
            "lowest_note" => musician.rearrange_by_lowest_note("F#min"),
            _ => panic!()
        }
    
        let chord_list_len = musician.chord_list.len();
        musician.set_probabilities();

        assert_eq!(chord_list_len, musician.chord_list.len());

        // sort the sub-arrays
        for col in musician.chord_table.iter_mut() {
            col.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
        }
    
        musician.chord_list.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
        
        let mut total_probability_2d: f32 = 0f32;
        let mut total_probability_1d: f32 = 0f32;
        for chord in musician.chord_list.iter() {
            total_probability_2d += chord.probability_2d;
            total_probability_1d += chord.probability_1d;
        }

        assert_eq!(&format!("{:.4}", total_probability_1d), "1.0000");
        assert_eq!(&format!("{:.4}", total_probability_2d), "1.0000");
    }
}