// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

/// This module implements two manufacturing modes:
///
/// # MasterToN
/// Manufacture `num` transactions from the master account
/// to `num` randomly created accounts, one each.
///
///   A -> B
///   A -> C
///   ... x `num`
///
///
/// # MasterTo1
/// Manufacture `num` transactions from the master account
/// to exactly one other randomly created account.
///
///   A -> B
///   A -> B
///   ... x `num`

use std::sync::Arc;

use log::info;
use sc_client::Client;
use sp_block_builder::BlockBuilder;
use sp_api::{ConstructRuntimeApi, ProvideRuntimeApi};
use sp_runtime::traits::{Block as BlockT, One};
use sp_runtime::generic::BlockId;

use crate::{RuntimeAdapter, create_block};

pub fn next<RA, Backend, Exec, Block, RtApi>(
	factory_state: &mut RA,
	client: &Arc<Client<Backend, Exec, Block, RtApi>>,
	version: u32,
	genesis_hash: <RA::Block as BlockT>::Hash,
	prior_block_hash: <RA::Block as BlockT>::Hash,
	prior_block_id: BlockId<Block>,
) -> Option<Block>
where
	Block: BlockT,
	Exec: sc_client::CallExecutor<Block, Backend = Backend> + Send + Sync + Clone,
	Backend: sc_client_api::backend::Backend<Block> + Send,
	Client<Backend, Exec, Block, RtApi>: ProvideRuntimeApi<Block>,
	<Client<Backend, Exec, Block, RtApi> as ProvideRuntimeApi<Block>>::Api:
		BlockBuilder<Block, Error = sp_blockchain::Error> +
		sp_api::ApiExt<Block, StateBackend = Backend::State>,
	RtApi: ConstructRuntimeApi<Block, Client<Backend, Exec, Block, RtApi>> + Send + Sync,
	RA: RuntimeAdapter,
{
	if factory_state.block_no() >= factory_state.num() {
		return None;
	}

	let block = create_block::<RA, _, _, _, _>(
		factory_state,
		&client,
		version,
		genesis_hash,
		prior_block_hash,
		prior_block_id,
	);

	factory_state.set_block_no(factory_state.block_no() + RA::Number::one());

	info!(
		"Created block {} with hash {}.",
		factory_state.block_no(),
		prior_block_hash,
	);

	Some(block)
}
