<script lang="ts">
	import Navbar from '$lib/Navbar.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	async function buy_stock(ticker: string) {
		await invoke('buy_stock', { ticker: ticker });
	}

	async function sell_stock(ticker: string) {
		await invoke('sell_stock', { ticker: ticker });
	}

	async function price_changes() {
		await invoke('price_changes');
	}

	setInterval(price_changes, 60000);
</script>

<button on:click={() => buy_stock('AAPL')}>Buy AAPL</button>
<button on:click={() => sell_stock('AAPL')}>Sell AAPL</button>

<Navbar />
