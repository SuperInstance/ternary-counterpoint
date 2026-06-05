//! # ternary-counterpoint
//!
//! Species counterpoint for ternary music.
//! Consonance, dissonance, voice leading, and the rules of two-part writing in Z₃.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

/// A ternary pitch: -1, 0, or +1 (subdominant, tonic, dominant)
pub type Pitch = i8;

/// A ternary interval: the difference between two pitches mod 3
pub fn interval(a: Pitch, b: Pitch) -> i8 {
    ((b - a) % 3 + 3) % 3  // 0=unison, 1=third, 2=fifth (tritone in ternary)
}

/// Consonance classification in ternary
/// Unison (0) = perfect consonance
/// Third (1) = imperfect consonance
/// Tritone (2) = dissonance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consonance {
    Perfect,    // interval 0 (unison)
    Imperfect,  // interval 1 (third)
    Dissonant,  // interval 2 (tritone)
}

pub fn classify_interval(a: Pitch, b: Pitch) -> Consonance {
    match interval(a, b) {
        0 => Consonance::Perfect,
        1 => Consonance::Imperfect,
        _ => Consonance::Dissonant,
    }
}

/// A voice: a sequence of ternary pitches
#[derive(Debug, Clone)]
pub struct Voice {
    pub notes: Vec<Pitch>,
}

impl Voice {
    pub fn new(notes: Vec<Pitch>) -> Self {
        Self { notes: notes.iter().map(|&n| n.clamp(-1, 1)).collect() }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    /// Get the melodic interval between consecutive notes
    pub fn melodic_intervals(&self) -> Vec<i8> {
        self.notes.windows(2)
            .map(|w| w[1] - w[0])
            .collect()
    }

    /// Is the melody "smooth" (all steps use modular ternary distance)?
    pub fn is_smooth(&self) -> bool {
        self.notes.windows(2).all(|w| {
            let d = (w[1] - w[0]).abs();
            d <= 1 || d == 2 // |2| wraps to 1 in Z₃
        })
    }

    /// Count melodic leaps (intervals > 1 or < -1)
    pub fn leap_count(&self) -> usize {
        self.melodic_intervals().iter().filter(|&&i| i.abs() > 1).count()
    }

    /// Range: difference between highest and lowest note
    pub fn range(&self) -> i8 {
        let max = self.notes.iter().max().copied().unwrap_or(0);
        let min = self.notes.iter().min().copied().unwrap_or(0);
        max - min
    }
}

/// Two-part counterpoint: cantus firmus + counterpoint voice
#[derive(Debug, Clone)]
pub struct Counterpoint {
    pub cantus_firmus: Voice,
    pub counter_voice: Voice,
}

impl Counterpoint {
    pub fn new(cf: Voice, cv: Voice) -> Self {
        assert_eq!(cf.len(), cv.len(), "Voices must have same length");
        Self { cantus_firmus: cf, counter_voice: cv }
    }

    /// Get all harmonic intervals (between the two voices at each position)
    pub fn harmonic_intervals(&self) -> Vec<i8> {
        self.cantus_firmus.notes.iter()
            .zip(self.counter_voice.notes.iter())
            .map(|(&a, &b)| interval(a, b))
            .collect()
    }

    /// Check for parallel perfect consonances (forbidden in strict counterpoint)
    /// Two consecutive perfect consonances with the same interval = parallel
    pub fn parallel_perfect(&self) -> Vec<usize> {
        let intervals = self.harmonic_intervals();
        let mut parallels = vec![];
        for i in 1..intervals.len() {
            if intervals[i] == 0 && intervals[i - 1] == 0 {
                parallels.push(i);
            }
        }
        parallels
    }

    /// Check for parallel fifths (interval 2 appearing consecutively)
    /// In ternary, the tritone (2) is the only "fifth-like" dissonance
    pub fn parallel_tritones(&self) -> Vec<usize> {
        let intervals = self.harmonic_intervals();
        let mut parallels = vec![];
        for i in 1..intervals.len() {
            if intervals[i] == 2 && intervals[i - 1] == 2 {
                parallels.push(i);
            }
        }
        parallels
    }

    /// Count dissonances (how many positions have dissonant intervals)
    pub fn dissonance_count(&self) -> usize {
        self.harmonic_intervals().iter().filter(|&&i| i == 2).count()
    }

    /// Consonance ratio: fraction of consonant positions
    pub fn consonance_ratio(&self) -> i8 {
        let total = self.cantus_firmus.len() as i8;
        if total == 0 { return 0; }
        let consonant = self.harmonic_intervals().iter().filter(|&&i| i != 2).count() as i8;
        (consonant * 3 / total).clamp(-1, 1)
    }

    /// Motion classification at position i
    pub fn motion_type(&self, i: usize) -> Motion {
        if i == 0 || i >= self.cantus_firmus.len() {
            return Motion::Static;
        }
        let cf_motion = self.cantus_firmus.notes[i] - self.cantus_firmus.notes[i - 1];
        let cv_motion = self.counter_voice.notes[i] - self.counter_voice.notes[i - 1];

        if cf_motion == 0 && cv_motion == 0 { Motion::Static }
        else if cf_motion == 0 { Motion::Oblique }
        else if cv_motion == 0 { Motion::Oblique }
        else if cf_motion > 0 && cv_motion > 0 { Motion::Parallel }
        else if cf_motion < 0 && cv_motion < 0 { Motion::Parallel }
        else { Motion::Contrary }
    }

    /// Full counterpoint analysis
    pub fn analyze(&self) -> CounterpointAnalysis {
        CounterpointAnalysis {
            harmonic_intervals: self.harmonic_intervals(),
            parallel_perfect: self.parallel_perfect(),
            parallel_tritones: self.parallel_tritones(),
            dissonance_count: self.dissonance_count(),
            consonance_ratio: self.consonance_ratio(),
            cf_melodic: self.cantus_firmus.melodic_intervals(),
            cv_melodic: self.counter_voice.melodic_intervals(),
        }
    }

    /// Is this valid first-species counterpoint?
    /// Rules: (1) begin and end on perfect consonance,
    ///        (2) no parallel perfect consonances,
    ///        (3) mostly consonant
    pub fn is_valid_species1(&self) -> bool {
        if self.cantus_firmus.len() < 2 { return false; }
        // Begin on unison
        if interval(self.cantus_firmus.notes[0], self.counter_voice.notes[0]) != 0 {
            return false;
        }
        // End on unison
        let n = self.cantus_firmus.len();
        if interval(self.cantus_firmus.notes[n-1], self.counter_voice.notes[n-1]) != 0 {
            return false;
        }
        // No parallel perfect consonances
        if !self.parallel_perfect().is_empty() {
            return false;
        }
        // Mostly consonant
        self.dissonance_count() * 2 < self.cantus_firmus.len()
    }
}

/// Motion types in two-part counterpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    Parallel,
    Contrary,
    Oblique,
    Static,
}

/// Complete counterpoint analysis
#[derive(Debug, Clone)]
pub struct CounterpointAnalysis {
    pub harmonic_intervals: Vec<i8>,
    pub parallel_perfect: Vec<usize>,
    pub parallel_tritones: Vec<usize>,
    pub dissonance_count: usize,
    pub consonance_ratio: i8,
    pub cf_melodic: Vec<i8>,
    pub cv_melodic: Vec<i8>,
}

/// Generate a simple counterpoint voice above a cantus firmus
/// using first-species rules (note-against-note, consonant intervals)
pub fn generate_first_species(cf: &Voice) -> Voice {
    let mut cv = vec![];
    for (i, &cf_note) in cf.notes.iter().enumerate() {
        let counter_note = if i == 0 || i == cf.notes.len() - 1 {
            // Begin and end on unison
            cf_note
        } else {
            // Try to use contrary motion and consonant intervals
            let prev_cf = cf.notes[i - 1];
            let cf_direction = cf_note - prev_cf;
            // Move in opposite direction, prefer consonance
            let cv_direction = -cf_direction;
            let candidate = cf_note + cv_direction;
            candidate.clamp(-1, 1)
        };
        cv.push(counter_note);
    }
    Voice::new(cv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval() {
        assert_eq!(interval(0, 0), 0); // unison
        assert_eq!(interval(0, 1), 1); // third
        assert_eq!(interval(0, -1), 2); // tritone
        assert_eq!(interval(-1, 1), 2);
    }

    #[test]
    fn test_consonance() {
        assert_eq!(classify_interval(0, 0), Consonance::Perfect);
        assert_eq!(classify_interval(0, 1), Consonance::Imperfect);
        assert_eq!(classify_interval(0, -1), Consonance::Dissonant);
    }

    #[test]
    fn test_voice_smooth() {
        let v = Voice::new(vec![0, 1, 0, -1, 0]);
        assert!(v.is_smooth());
    }

    #[test]
    fn test_voice_ternary_always_smooth() {
        // In ternary, all steps are within {-1,0,1}, so all melodies are smooth
        let v = Voice::new(vec![0, 1, -1, 1, 0]);
        assert!(v.is_smooth()); // This is actually a deep insight!
    }

    #[test]
    fn test_voice_range() {
        let v = Voice::new(vec![-1, 0, 1, 0, -1]);
        assert_eq!(v.range(), 2);
    }

    #[test]
    fn test_counterpoint_harmonic_intervals() {
        let cf = Voice::new(vec![0, 1, 0, -1, 0]);
        let cv = Voice::new(vec![0, 0, 1, 0, 0]);
        let cp = Counterpoint::new(cf, cv);
        let hi = cp.harmonic_intervals();
        assert_eq!(hi.len(), 5);
    }

    #[test]
    fn test_counterpoint_parallels() {
        let cf = Voice::new(vec![0, 1, 0]);
        let cv = Voice::new(vec![0, 1, 0]);
        let cp = Counterpoint::new(cf, cv);
        // Both transitions are parallel unisons
        assert_eq!(cp.parallel_perfect(), vec![1, 2]);
    }

    #[test]
    fn test_counterpoint_consonance_ratio() {
        let cf = Voice::new(vec![0, 0, 0]);
        let cv = Voice::new(vec![0, 1, -1]);
        let cp = Counterpoint::new(cf, cv);
        // 2/3 consonant → ratio = 2 (i.e. 2/3 * 3)
        assert!(cp.consonance_ratio() >= 1);
    }

    #[test]
    fn test_motion_types() {
        let cf = Voice::new(vec![0, 1, 0]);
        let cv = Voice::new(vec![0, 0, 1]);
        let cp = Counterpoint::new(cf, cv);
        assert_eq!(cp.motion_type(1), Motion::Oblique); // cf goes up, cv stays
        assert_eq!(cp.motion_type(2), Motion::Contrary); // cf goes down, cv goes up
    }

    #[test]
    fn test_generate_first_species() {
        let cf = Voice::new(vec![0, 1, 0, -1, 0]);
        let cv = generate_first_species(&cf);
        assert_eq!(cv.len(), cf.len());
        // Should begin and end on unison
        assert_eq!(cv.notes[0], cf.notes[0]);
        assert_eq!(cv.notes[4], cf.notes[4]);
    }

    #[test]
    fn test_generate_valid() {
        let cf = Voice::new(vec![0, 1, 0, -1, 0]);
        let cv = generate_first_species(&cf);
        let cp = Counterpoint::new(cf, cv);
        // Generated counterpoint should be valid
        assert!(cp.is_valid_species1());
    }

    #[test]
    fn test_analyze() {
        let cf = Voice::new(vec![0, 1, 0, -1, 0]);
        let cv = Voice::new(vec![0, 0, 1, 0, 0]);
        let cp = Counterpoint::new(cf, cv);
        let analysis = cp.analyze();
        assert_eq!(analysis.harmonic_intervals.len(), 5);
    }

    #[test]
    fn test_dissonance_count() {
        let cf = Voice::new(vec![0, 0, 0]);
        let cv = Voice::new(vec![-1, 0, -1]); // tritone, unison, tritone
        let cp = Counterpoint::new(cf, cv);
        assert_eq!(cp.dissonance_count(), 2);
    }
}
