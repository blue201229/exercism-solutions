#[derive(Debug)]
pub struct HighScores{
    scores:Vec<u32>,
    sorted:Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut sorted = scores.to_vec();
        sorted.sort_by(|a, b| b.cmp(a));
        Self{scores:scores.to_vec(),sorted:sorted}
    }

    pub fn scores(&self) -> &[u32] {
         self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            self.scores.last().copied()
        }
        
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.sorted.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let min_num = self.sorted.len().min(3);
        self.sorted
        .get(0..min_num)
        .unwrap_or_default()
        .to_vec()
    }
}
