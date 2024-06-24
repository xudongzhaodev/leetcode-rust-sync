use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_count = HashMap::new();
        for c in magazine.chars() {
            *char_count.entry(c).or_insert(0) +=1;
        }

        for c in ransom_note.chars() {
            let count = char_count.entry(c).or_insert(0);
            if *count ==0 {
                return false;
            }
            *count-=1;
        }
        true
    }
}
