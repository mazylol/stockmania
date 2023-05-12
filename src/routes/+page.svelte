<script>
	import Greet from '../lib/Greet.svelte';
</script>

<h1 id="pixelfont" class="text-8xl text-center mt-[25vh]"><span class="text-green-400">Stock</span><span class="text-red-600">Mania</span></h1>

<p class="text-center text-2xl mt-4">A stock market game where you get to control the economy</p>

<div class="flex justify-center mt-8">
    <a href="/game" class="bg-green-400 hover:bg-green-500 text-white font-bold py-2 px-4 rounded">Play</a>
</div>

<style>
    @font-face {
        font-family: PixeloidSans;
        font-style: normal;
        font-weight: 400;
        src: url(/fonts/PixeloidSans-mLxMm.ttf);

    }

    #pixelfont {
        font-family: PixeloidSans;
    }
</style>