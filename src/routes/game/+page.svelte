<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';

	import { price_changes, sell_stock, buy_stock, get_data } from '$lib/functions';

	type Data = {
		stocks: {
			ticker: string;
			price: number;
			owned: number;
		}[];
		balance: number;
	};

	let data: Data = get_data();

	setInterval(async () => {
		await price_changes();
		data = await get_data();
	}, 30000);
</script>

{#await data}
	<p>loading...</p>
{:then data}
	<h1 class="text-center text-4xl font-bold my-8">
		Cash: ${Math.round((data.balance + Number.EPSILON) * 100) / 100}
	</h1>

	<div class="grid gap-4 grid-cols-2 mx-4">
		{#each data.stocks as stock}
			<div class="bg-gray-200 rounded-md">
				<h1 class="text-center text-2xl font-bold">{stock.ticker}</h1>
				<h3 class="text-center text-2xl font-semibold">
					${Math.round((stock.price + Number.EPSILON) * 100) / 100}
				</h3>
				<button
					class="bg-green-500 rounded-md p-2 m-2"
					on:click={async () => {
						await buy_stock(stock.ticker);
						data = await get_data();
					}}>Buy</button
				>
				<button
					class="bg-red-500 rounded-md p-2 m-2"
					on:click={async () => {
						await sell_stock(stock.ticker);
						data = await get_data();
					}}>Sell</button
				>
				<h3 class="text-center text-2xl font-semibold">Owned: {stock.owned}</h3>
			</div>
		{/each}
	</div>
{:catch error}
	<p>{error.message}</p>
{/await}

<Navbar />
