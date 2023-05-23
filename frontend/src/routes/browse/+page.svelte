<script>
import theme from "../../theme";
import SearchIcon from "../../assets/SearchIcon.svelte";
import FilterButton from "../../components/FilterButton/FilterButton.svelte";

let toggled = false;

const handleClick = () => {
    toggled = !toggled;
    console.log(toggled)
}
</script>

<div class="browse-container">
    <div class="control-container">
        <FilterButton paddingLR={"2vw"} fontSize={"1.25rem"} />
        <input class="search" type="search" placeholder="Search" 
            style="--bg-color: {theme.colors.neutral[800]}"
        />
        <SearchIcon />
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
</style>


