<script>
	import Greet from '$lib/Greet.svelte';
	import Navbar from '$lib/Navbar.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	async function print_stocks() {
		await invoke('print_stocks');
	}
</script>

<p>{print_stocks()}</p>

<Navbar />
