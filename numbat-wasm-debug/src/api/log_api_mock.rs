use crate::TxContext;
use crate::TxLog;
use numbat_wasm::{api::LogApi, types::ArgBuffer};

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
impl LogApi for TxContext {
	fn write_event_log(&self, topics_buffer: &ArgBuffer, data: &[u8]) {
		let arg_data_buffer = topics_buffer.arg_data();
		let arg_data_lengths = topics_buffer.arg_lengths();

		// identifier is the first arg data
		let identifier_len = arg_data_lengths[0];
		let identifier = arg_data_buffer[0..identifier_len].to_vec();

		let mut current_index = identifier_len;
		let mut topics = Vec::new();

		// we already processed the first data arg, so we skip it
		for arg_len in arg_data_lengths[1..].iter() {
			let topic = arg_data_buffer[current_index..(current_index + arg_len)].to_vec();
			topics.push(topic);

			current_index += arg_len;
		}

		let mut tx_output_cell = self.tx_output_cell.borrow_mut();
		tx_output_cell.result.result_logs.push(TxLog {
			address: self.tx_input_box.to.clone(),
			identifier,
			topics,
			data: data.to_vec(),
		});
	}

	fn write_legacy_log(&self, topics: &[[u8; 32]], data: &[u8]) {
		let identifier = topics[0].to_vec();
		let topics_vec = topics[1..].iter().map(|array| array.to_vec()).collect();

		let mut tx_output_cell = self.tx_output_cell.borrow_mut();
		tx_output_cell.result.result_logs.push(TxLog {
			address: self.tx_input_box.to.clone(),
			identifier,
			topics: topics_vec,
			data: data.to_vec(),
		});
	}
}