<script context="module">
    import { writable } from 'svelte/store';
    import {onMount} from "svelte";

    let gameServers = writable([]);
    let intervalId;

    export async function triggerServerFetch() {
        try {
            const response = await fetch('http://localhost:8000/fetch-servers', {
                method: 'POST'
            });
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
            console.log('Server fetch triggered successfully');
        } catch (error) {
            console.error('Error triggering server fetch:', error);
        }
    }

    export async function updateGameServers() {
        try {
            const response = await fetch('http://localhost:8000/new-servers', {
                method: 'GET'
            });
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const newData = await response.json();

            // Use the update method to append new data to the existing array
            gameServers.update(currentData => {
                // Concatenate the new data with the current data
                return [...currentData, ...newData];
            });
        } catch (error) {
            console.error('Failed to fetch game servers:', error);
        }
    }

    function truncateString(str, num) {
        if (str.length > num) {
            return str.slice(0, num) + "...";
        } else {
            return str;
        }
    }

    // onMount(async () => {
    //     try {
    //         const response = await fetch('http://localhost:8000/fetch-servers', {
    //             method: 'POST'
    //         });
    //         if (!response.ok) {
    //             throw new Error(`HTTP error! Status: ${response.status}`);
    //         }
    //         console.log('Server fetch triggered successfully');
    //     } catch (error) {
    //         console.error('Error triggering server fetch:', error);
    //     }
    // });

    // onDestroy(() => {
    //     clearInterval(intervalId);
    // });

</script>

<style>
    .list-container {
        font-family: 'Arial', sans-serif;
        background-color: #1c1c1e;
        color: #f1f1f1;
        width: 100%;
        margin-top: 10px;
        box-shadow: 0 4px 8px 0 rgba(0,0,0,0.5);
        border-radius: 8px;
        overflow: hidden;
    }

    .list-container ul {
        list-style-type: none;
        padding: 0;
    }

    .server-header {
        font-weight: bold;
        margin: 5px;
    }

    .server-details {
        margin-bottom: 1em;
    }

    .server-header, .server-details {
        display: flex;
        justify-content: space-between;
        margin-left: 1em;
        margin-right: 1em;
    }

    .server-details:hover {
        background-color: #353535; /* Slight highlight on hover */
    }

    .server-details span {
        display: flex;
    }

    .server-header span, .server-details span {
        flex: 1;
        justify-content: space-between;
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
    }

    .server-details span:not(:last-child):after,
    .server-header span:not(:last-child):after {
        margin: 0 10px;
    }

</style>

<div class="list-container">
    <ul>
        <li class="server-header">
            <span class="header-name">Name</span>
            <span class="header-map">Map</span>
            <span class="header-players">Players</span>
        </li>
        {#each $gameServers as server}
            <li class="server-details">
                <span class="server-name">{truncateString(server.name, 50)}</span>
                <span class="server-map">{server.map}</span>
                <span class="server-players">{server.player_count}/{server.max_players}</span>
            </li>
        {/each}
    </ul>
</div>


