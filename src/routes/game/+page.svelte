<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';

	import { price_changes, sell_stock, buy_stock, get_data } from '$lib/functions';

	let data: any = get_data();

	setInterval(() => {
		price_changes();
		data = get_data();
	}, 5000);
</script>

<button
	on:click={() => {
		buy_stock('AAPL');
		data = get_data();
	}}>Buy AAPL</button
>
<button
	on:click={() => {
		sell_stock('AAPL');
		data = get_data();
	}}>Sell AAPL</button
>

{#await data}
	<p>loading...</p>
{:then data}
	{#each data.stocks as stock}
		<p>{stock.ticker}: {stock.price}</p>
	{/each}
	<p>{data.balance}</p>
{:catch error}
	<p>{error.message}</p>
{/await}

<Navbar />
