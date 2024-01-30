<script>
    import { updateGameServers } from '$lib/GameServers.svelte';
</script>

<style>
    button {
        font-family: "Good Times Rg", serif;
        color: #000000;
        background-color: #fef901;
    }
</style>

<button on:click={updateGameServers}>UPDATE</button>
