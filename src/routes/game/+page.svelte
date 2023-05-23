<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	
	import { price_changes, sell_stock, buy_stock } from "$lib/functions"

	setInterval(price_changes, 60000);
</script>

<button on:click={() => buy_stock('AAPL')}>Buy AAPL</button>
<button on:click={() => sell_stock('AAPL')}>Sell AAPL</button>

<Navbar />
