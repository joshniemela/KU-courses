<script lang="ts">
    import Github from "../../assets/Github.svelte";
</script>

<footer class="mt-10 px-2 md:w-1/2 md:px-0 mx-auto py-4 border-t-2 border-kuGray flex justify-between items-center">
    <div class="text-sm">
        <span class="block">
            Developed by
            <a class="underline font-bold" href="https://jniemela.dk">
                Joshua Niemelä
            </a>
            and
            <a class="underline font-bold"
                href="https://github.com/joshniemela/KU-courses/graphs/contributors"
            >
                others
            </a>
        </span>
        <span class="block">
            <a class="underline font-bold" href="mailto:josh@jniemela.dk">josh@jniemela.dk</a>
        </span>
    </div>
    <div class="flex space-x-4 text-sm">
        
        <a class="block animated-icon" target="_blank" href="https://github.com/joshniemela/KU-courses">
            <Github classes="w-6 h-6"></Github>
        </a>
    </div>
</footer>

<style scoped>
    .animated-icon {
        @apply duration-300;

        transform: scale(1);
    }

    .animated-icon:hover {
        transform: scale(1.25);
    }
</style>