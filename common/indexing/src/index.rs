// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use common_datablocks::DataBlock;
use common_exception::Result;
use common_planners::PlanNode;

use crate::MinMaxIndex;
use crate::SparseIndex;

#[derive(Debug, PartialEq)]
pub struct IndexSchema {
    // The column name of the index.
    pub col: String,
    pub min_max: MinMaxIndex,
    pub sparse: SparseIndex,
}

pub trait Index {
    /// Create index from blocks.
    /// Each block is one row group of a parquet file and sorted by primary key.
    /// For example:
    /// parquet.file
    /// | sorted-block | sorted-block | ... |
    fn create_index(&self, keys: &[String], blocks: &[DataBlock]) -> Result<Vec<IndexSchema>>;

    // Search parts by plan.
    fn search_index(&self, plan: &PlanNode) -> Result<()>;
}
