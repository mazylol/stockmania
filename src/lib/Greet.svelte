<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let name = '';
    let greetMsg = '';

    async function greet() {
        greetMsg = await invoke('greet', { name });
    }
</script>

<div class="ml-2">
    <input class="rounded-md" id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
    <button class="hover:bg-blue-500 hover:text-gray-200 rounded-md transition-all duration-200" on:click="{greet}">Greet</button>
    <p>{greetMsg}</p>
</div>