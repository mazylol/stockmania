import { invoke } from '@tauri-apps/api/tauri';

export async function get_data() {
	return await invoke('get_data');
}

export async function buy_stock(ticker: string) {
	await invoke('buy_stock', { ticker });
}

export async function sell_stock(ticker: string) {
	await invoke('sell_stock', { ticker });
}

export async function price_changes() {
	await invoke('price_changes');
}
