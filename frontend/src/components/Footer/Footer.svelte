<script lang="ts">
    import Facebook from "../../assets/Facebook.svelte";
import Github from "../../assets/Github.svelte";
    import { obfuscateEmail } from "../../stores";
    import { onMount } from "svelte";

    export let additionalClasses: string = "";

    let email_fn = () => "";

    onMount(() => {
        email_fn = obfuscateEmail("josh@jniemela.dk");
    });
</script>

<footer
    class="mt-2 p-4 border-t-2 border-kuGray flex justify-between items-center {additionalClasses}"
>
    <div class="text-sm">
        <span class="block">
            Developed by
            <a class="underline font-bold" href="https://jniemela.dk">
                Joshua Niemelä
            </a>
            and
            <a
                class="underline font-bold"
                href="https://github.com/joshniemela/KU-courses/graphs/contributors"
            >
                others
            </a>
        </span>
        <span class="block">
            <a class="underline font-bold" href="mailto:{email_fn()}">
                {email_fn()}
            </a>
        </span>
    </div>
    <div class="flex space-x-4 text-sm">
        <a
            class="block animated-icon"
            target="_blank"
            href="https://github.com/joshniemela/KU-courses"
        >
            <Github classes="size-8" />
        </a>

        <a
            href="https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fdisku.jniemela.dk"
            rel="noopener"
            target="_blank"
            class="block animated-icon"
        >
            <Facebook classes="size-8"/>
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
