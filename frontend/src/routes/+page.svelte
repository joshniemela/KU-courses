<script>
import theme from '../theme'
import SearchIcon from '../assets/SearchIcon.svelte';
import FilterButton from '../components/FilterButton/FilterButton.svelte';
import { navigate } from 'svelte-navigator';
/**
* navigates to the /browse route.
* @function submit 
* @param {event} event event: the event emitted by the component on click / enter
*/
function submit(event) {
    if (event.key === 'Enter') {
        console.log("Submit")
        navigate('/browse')
        location.reload()
    }
}

</script>
    <div class="content">
        <h1 class="title" style="--font-color: {theme.colors.brand[500]}">KU Courses (WIP)</h1>

        <!-- Container responsible for the search area --> 
        <div class="search-container">
            <FilterButton />
            <input class="search" type="search" placeholder="Search"
                style="
                --text-color: {theme.colors.brand[200]};
                --search-bg-color: {theme.colors.neutral[800]}
                "
                on:keydown={submit}
            />
            <a class="search-icon-ref" href="/browse">
            <SearchIcon />
            </a>
        </div>

        <a href="/browse">
        <button class="view-all-button"
            style="
                --bg-color: {theme.colors.neutral[900]};
                --text-color: {theme.colors.brand[200]}
            "
        >View all (WIP)</button>
        </a>
</div>

<style scoped>
.title {
    color: var(--font-color);
    margin-bottom: 2vh;
}

.content {
    display: flex;
    height: 100vh;
    width: 100%;
    margin-right: 8vw; /* should be the same as navbar-container width */
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.search-container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    margin-bottom: 2vh;
    height: 5vh;
    width: 30vw;
}


.search {
    font-size: 1.5rem;
    border: 0;
    width: 100%;
    height: 100%;
    padding-left: 1vw;
    color: var(--text-color);
    background-color: var(--search-bg-color);
}

.search:focus {
    outline: none !important;
    border:2px solid var(--text-color);
}

.view-all-button {
    background: none;
    font-size: 1rem;
    border: 0;
    color: var(--text-color);
    width: 8vw;
    background-color: var(--bg-color);
    transition: ease-in-out 0.1s;
}

.search-icon-ref {
    height: 5vh;
}
</style>
