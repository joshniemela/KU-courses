<script>
    let isOpen = false;

    const toggleDropdown = () => {
        isOpen = !isOpen;
    };

    const closeDropdown = () => {
        isOpen = false;
    };

    const changelogItems = [
        {
            date: "2024-01-13",
            changes: [
                "Fixed opening in new tab and copying of course links",
                "Fixed back button when coming from an external page",
            ],
        },
        {
            date: "2024-02-05",
            changes: [
                "Fixed that performing a vector search and a normal filter will destroy results that should appear.",
                "Fixed that statistics weren't being updated",
            ],
        },
    ];
</script>

<div class="p-2 relative text-xl font-bold" on:mouseleave={closeDropdown}>
    <button on:click={toggleDropdown}>Changelog</button>
    {#if isOpen}
        <div class="text-m font-normal">
            {#each changelogItems as { date, changes }}
                <div class="">
                    <strong>{date}</strong>
                    <ul>
                        {#each changes as change}
                            <li>{change}</li>
                        {/each}
                    </ul>
                </div>
            {/each}
        </div>
    {/if}
</div>
