mod save_file;

use crate::*;
use save_file::*;
use std::{collections::HashMap, path::PathBuf};

pub struct ChapterSolution {
    pub completion_status: ChapterCompletionStatus,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Copy, Clone)]
pub enum ChapterCompletionStatus {
    NotStarted,
    InProgress,
    Completed,
}

impl Chapter {
    // pub fn load_solution(&self) -> ChapterSolution {
    //     let save_file_path = todo!()
    //
    //     if let Some(save_file) = SolutionSaveFile::from_path(save_file_path) {
    //         ChapterSolution {
    //             blocks: save_file.blocks,
    //             completion_status: self.completion_status,
    //         }
    //     } else {
    //         self.new_solution()
    //     }
    // }

    /// Creates a blank solution without checking or loading any save_file
    pub fn new_solution(&self) -> ChapterSolution {
        ChapterSolution {
            blocks: vec![],
            completion_status: ChapterCompletionStatus::NotStarted,
        }
    }
}
