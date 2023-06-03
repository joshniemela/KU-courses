<script>
import theme from "../../theme";
import SearchIcon from "../../assets/SearchIcon.svelte";
import FilterButton from "../../components/FilterButton/FilterButton.svelte";
import Dk from "../../assets/Dk.svelte";
import overview from "../../mocking/overview.json";
import OverviewCard from "../../components/OverviewCard/OverviewCard.svelte";
console.log(overview)
</script>
<div class="browse-container">
    <div class="control-container">
        <FilterButton paddingLR={"2vw"} fontSize={"1.25rem"} />
        <input class="search" type="search" placeholder="Search" 
            style="--bg-color: {theme.colors.neutral[800]}"
        />
        <SearchIcon />
    </div>
    <div class="card-container">
        {#each overview as card, i}
            <OverviewCard stagger={i} data={card} />
        {/each}
    </div>
</div>

<style scoped>
.browse-container {
    height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.control-container {
    margin-top: 1vh;
    height: 4vh;
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
}

.search {
    height: 100%;
    width: 30vw;
    max-width: 800px;
    margin-left: 1vw;
    font-size: 1.25rem;
    padding-left: 1vw;
    border: 0;
    background-color: var(--bg-color);

}

.card-container {
    height: 100%;
    width:96%;
    margin: 2%;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-auto-rows: 33vh;
    justify-content: center;
    align-items: center;
    gap: 2vh;
}

</style>


