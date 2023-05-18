<script lang="ts">
	import Navbar from '$lib/Navbar.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	async function buy_stock(ticker: string) {
		await invoke('buy_stock', { ticker: ticker });
	}

	async function sell_stock(ticker: string) {
		await invoke('sell_stock', { ticker: ticker });
	}
</script>

<button on:click={() => buy_stock('AAPL')}>Buy AAPL</button>
<button on:click={() => sell_stock('AAPL')}>Sell AAPL</button>

<Navbar />
