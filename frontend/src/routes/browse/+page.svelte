<script>
import theme from "../../theme";
import SearchIcon from "../../assets/SearchIcon.svelte";
import FilterButton from "../../components/FilterButton/FilterButton.svelte";

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
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {0};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {1};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {2};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {3};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {4};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {5};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {6};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {7};
                "
        >

        </div>
        <div class="card"
            style="
                --bg-color: {theme.colors.neutral[800]};
                --stagger: {8};
                "
        >

        </div>
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
    margin: 2vh;
    display: grid;
    grid-template: 1fr 1fr 1fr / 1fr 1fr 1fr;
    justify-content: center;
    align-items: center;
    gap: 2vh;
}

.card {
    height: 100%;
    width: 100%; 
    background-color: var(--bg-color);
    border-radius: 10px;
    box-shadow: 0 2.8px 2.2px rgba(0, 0, 0, 0.15);
    transition: ease-in-out 0.2s;
    opacity: 0%;
    animation: fadeIn 0.5s calc(var(--stagger)*0.05s);
    animation-fill-mode: forwards;
}
.card:hover {
    scale: 1.02;
    box-shadow: 0 2.8px 2.2px rgba(0, 0, 0, 0.25);
    transition: ease-in-out 0.2s;
}

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}


</style>


