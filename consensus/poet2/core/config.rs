/*
 * Copyright 2018 Intel Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

use std::fmt;

use poet2::{
    Config as Poet2Config,
    storage::{MemStorage},
};

use sawtooth_sdk::consensus::{engine::{BlockId, PeerId}, service::Service};

pub struct Poet2EngineConfig {
    pub poet2: Poet2Config,
    pub storage: MemStorage,
}

impl Default for Poet2EngineConfig {
    fn default() -> Self {
        let mut poet2 = Poet2Config::default();
        poet2.block_claim_delay = 1;
        poet2.initial_wait_time = 3000.0;
        poet2.registration_retry_delay = 10;
        poet2.target_wait_time = 20.0;
        poet2.min_wait_time = 1.0;
        poet2.max_wait_time = 60.0;
        poet2.ztest_max_win_deviation = 3.075;
        poet2.min_win_count = 3;

        Poet2EngineConfig {
            poet2,
            storage: MemStorage::new(),
        }
    }
}

impl fmt::Debug for Poet2EngineConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Poet2EngineConfig {{ poet2: {{ block_claim_delay: {}, initial_wait_time: {}, 
              registration_retry_delay: {}, target_wait_time: {},
              min_wait_time: {}, max_wait_time: {}, ztest_max_win_deviation: {},
              min_win_count: {}
             }}, storage: MemStorage }}",
            self.poet2.block_claim_delay,
            self.poet2.initial_wait_time,
            self.poet2.registration_retry_delay,
            self.poet2.target_wait_time,
            self.poet2.min_wait_time,
            self.poet2.max_wait_time,
            self.poet2.ztest_max_win_deviation,
            self.poet2.min_win_count
        )
    }
}
