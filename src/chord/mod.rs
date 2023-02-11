use itertools::Itertools;

use crate::duration::Duration;
use crate::leaf::Leaf;
use crate::notehead::Notehead;
use crate::pitch::Pitch;
use crate::to_lilypond::ToLilypond;

type NoteheadList = Vec<Notehead>;

#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chord {
    noteheads: NoteheadList,
    written_duration: Duration,
}

impl Chord {
    pub fn new(pitches: &[Pitch], written_duration: Duration) -> Self {
        let noteheads = Self::pitches_to_notehead_list(pitches);
        Self {
            noteheads,
            written_duration,
        }
    }

    #[must_use]
    pub fn written_pitches(&self) -> Vec<Pitch> {
        self.noteheads
            .iter()
            .map(|&n| n.written_pitch())
            .collect::<Vec<Pitch>>()
    }

    pub fn insert(&mut self, pitch: Pitch) {
        let mut pitches = self.written_pitches();
        pitches.insert(0, pitch);
        pitches.sort();
        self.noteheads = Self::pitches_to_notehead_list(&pitches);
    }

    fn pitches_to_notehead_list(pitches: &[Pitch]) -> NoteheadList {
        pitches
            .iter()
            .map(|&p| Notehead::new(p))
            .collect::<NoteheadList>()
    }
}

impl Leaf for Chord {}

impl ToLilypond for Chord {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        match self.written_duration.to_lilypond() {
            Ok(duration_lilypond) => Ok(format!(
                "<\n{}\n>{duration_lilypond}",
                noteheads_to_lily(&self.noteheads),
            )),
            Err(err) => Err(err),
        }
    }
}

fn noteheads_to_lily(noteheads: &NoteheadList) -> String {
    noteheads
        .iter()
        .map(|n| format!("  {}", n.to_lilypond().unwrap()))
        .join("\n")
}

#[cfg(test)]
mod tests;