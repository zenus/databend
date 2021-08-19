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
use common_datavalues::prelude::*;
use common_datavalues::DataField;
use common_datavalues::DataSchemaRefExt;
use common_datavalues::DataType;
use common_exception::Result;

use crate::Index;
use crate::IndexSchema;
use crate::Indexer;
use crate::MinMaxIndex;
use crate::SparseIndex;
use crate::SparseIndexValue;

#[test]
fn test_indexer() -> Result<()> {
    let schema = DataSchemaRefExt::create(vec![
        DataField::new("name", DataType::Utf8, true),
        DataField::new("age", DataType::Int32, false),
    ]);

    let block1 = DataBlock::create_by_array(schema.clone(), vec![
        Series::new(vec!["jack", "ace", "bohu"]),
        Series::new(vec![11, 6, 24]),
    ]);

    let block2 = DataBlock::create_by_array(schema.clone(), vec![
        Series::new(vec!["xjack", "xace", "xbohu"]),
        Series::new(vec![11, 6, 24]),
    ]);

    let indexer = Indexer::create();
    let actual =
        indexer.create_index(&["name".to_string(), "age".to_string()], &[block1, block2])?;
    //let expected = [IndexSchema { col: "name", min_max: MinMaxIndex { min: jack, max: xbohu }, sparse: SparseIndex { values: [SparseIndexValue { min: jack, max: bohu, page_no: Some(0) }, SparseIndexValue { min: xjack, max: xbohu, page_no: Some(1) }] } }, IndexSchema { col: "age", min_max: MinMaxIndex { min: 11, max: 24 }, sparse: SparseIndex { values: [SparseIndexValue { min: 11, max: 24, page_no: Some(0) }, SparseIndexValue { min: 11, max: 24, page_no: Some(1) }] } }]";
    let expected = vec![
        IndexSchema {
            col: "name".to_string(),
            min_max: MinMaxIndex {
                min: DataValue::Utf8(Some("jack".to_string())),
                max: DataValue::Utf8(Some("xbohu".to_string())),
            },
            sparse: SparseIndex {
                values: vec![
                    SparseIndexValue {
                        min: DataValue::Utf8(Some("jack".to_string())),
                        max: DataValue::Utf8(Some("bohu".to_string())),
                        page_no: Some(0),
                    },
                    SparseIndexValue {
                        min: DataValue::Utf8(Some("xjack".to_string())),
                        max: DataValue::Utf8(Some("xbohu".to_string())),
                        page_no: Some(1),
                    },
                ],
            },
        },
        IndexSchema {
            col: "age".to_string(),
            min_max: MinMaxIndex {
                min: DataValue::Int32(Some(11)),
                max: DataValue::Int32(Some(24)),
            },
            sparse: SparseIndex {
                values: vec![
                    SparseIndexValue {
                        min: DataValue::Int32(Some(11)),
                        max: DataValue::Int32(Some(24)),
                        page_no: Some(0),
                    },
                    SparseIndexValue {
                        min: DataValue::Int32(Some(11)),
                        max: DataValue::Int32(Some(24)),
                        page_no: Some(1),
                    },
                ],
            },
        },
    ];
    assert_eq!(actual, expected);
    Ok(())
}
